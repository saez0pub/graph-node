use super::{Blockchain, RuntimeAdapter};

/// A [`RuntimeAdapter`] implementor without any host functions.
#[derive(Debug, Clone, Default)]
pub struct EmptyRuntimeAdapter;

impl<C> RuntimeAdapter<C> for EmptyRuntimeAdapter
where
    C: Blockchain,
{
    fn host_fns(
        &self,
        _ds: &<C as Blockchain>::DataSource,
    ) -> Result<Vec<super::HostFn>, anyhow::Error> {
        Ok(vec![])
    }
}
