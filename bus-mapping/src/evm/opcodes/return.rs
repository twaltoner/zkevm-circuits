use super::Opcode;
use crate::circuit_input_builder::{CircuitInputStateRef, ExecStep};
use crate::error::{get_step_reported_error, ExecError};
use crate::Error;
use eth_types::GethExecStep;

/// Placeholder structure used to implement [`Opcode`] trait over it
/// corresponding to the [`OpcodeId::RETURN`](crate::evm::OpcodeId::RETURN).
#[derive(Debug, Copy, Clone)]
pub(crate) struct Return;

impl Opcode for Return {
    fn gen_associated_ops(
        state: &mut CircuitInputStateRef,
        geth_steps: &[GethExecStep],
    ) -> Result<Vec<ExecStep>, Error> {
        let geth_step = &geth_steps[0];
        let exec_step = state.new_step(geth_step)?;
        // handle error condition
        if let Some(error) = geth_step.clone().error {
            let mut exec_step = state.new_step(geth_step)?;
            let execution_error: ExecError = get_step_reported_error(&geth_step.op, &error);
            log::warn!("geth error {} occurred in Return", error);
            exec_step.error = Some(execution_error);
            state.handle_return(geth_step)?;
            return Ok(vec![exec_step]);
        }
        // TODO: Generate associated operations of RETURN

        state.handle_return(geth_step)?;
        Ok(vec![exec_step])
    }
}
