use crate::{
    bundle_state::BundleStateWithReceipts, BlockExecutor, ExecutorFactory, PrunableBlockExecutor,
    StateProvider,
};
use parking_lot::Mutex;
use reth_interfaces::executor::BlockExecutionError;
use reth_primitives::{BlockNumber, BlockWithSenders, PruneModes, Receipt, U256};
use std::sync::Arc;
/// Test executor with mocked result.
#[derive(Debug)]
pub struct TestExecutor(pub Option<BundleStateWithReceipts>);

impl BlockExecutor for TestExecutor {
    fn execute(
        &mut self,
        _block: &BlockWithSenders,
        _total_difficulty: U256,
    ) -> Result<(), BlockExecutionError> {
        if self.0.is_none() {
            return Err(BlockExecutionError::UnavailableForTest)
        }
        Ok(())
    }

    fn execute_and_verify_receipt(
        &mut self,
        _block: &BlockWithSenders,
        _total_difficulty: U256,
    ) -> Result<(), BlockExecutionError> {
        if self.0.is_none() {
            return Err(BlockExecutionError::UnavailableForTest)
        }
        Ok(())
    }

    fn execute_transactions(
        &mut self,
        _block: &BlockWithSenders,
        _total_difficulty: U256,
    ) -> Result<(Vec<Receipt>, u64), BlockExecutionError> {
        Err(BlockExecutionError::UnavailableForTest)
    }

    fn take_output_state(self) -> BundleStateWithReceipts {
        self.0.unwrap_or_default()
    }

    fn size_hint(&self) -> Option<usize> {
        None
    }
}

impl PrunableBlockExecutor for TestExecutor {
    fn set_tip(&mut self, _tip: BlockNumber) {}

    fn set_prune_modes(&mut self, _prune_modes: PruneModes) {}
}

/// Executor factory with pre-set execution results.
#[derive(Clone, Debug, Default)]
pub struct TestExecutorFactory {
    exec_results: Arc<Mutex<Vec<BundleStateWithReceipts>>>,
}

impl TestExecutorFactory {
    /// Extend the mocked execution results
    pub fn extend(&self, results: Vec<BundleStateWithReceipts>) {
        self.exec_results.lock().extend(results);
    }
}

impl ExecutorFactory for TestExecutorFactory {
    type Executor = TestExecutor;

    fn with_state<SP: StateProvider>(&self, _sp: SP) -> Self::Executor {
        let exec_res = self.exec_results.lock().pop();
        TestExecutor(exec_res)
    }
}
