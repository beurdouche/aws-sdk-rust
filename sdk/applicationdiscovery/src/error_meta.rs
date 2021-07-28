// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
#[non_exhaustive]
#[derive(std::fmt::Debug)]
pub enum Error {
    AuthorizationErrorException(crate::error::AuthorizationErrorException),
    ConflictErrorException(crate::error::ConflictErrorException),
    HomeRegionNotSetException(crate::error::HomeRegionNotSetException),
    InvalidParameterException(crate::error::InvalidParameterException),
    InvalidParameterValueException(crate::error::InvalidParameterValueException),
    OperationNotPermittedException(crate::error::OperationNotPermittedException),
    ResourceInUseException(crate::error::ResourceInUseException),
    ResourceNotFoundException(crate::error::ResourceNotFoundException),
    ServerInternalErrorException(crate::error::ServerInternalErrorException),
    Unhandled(Box<dyn std::error::Error + Send + Sync + 'static>),
}
impl std::fmt::Display for Error {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Error::AuthorizationErrorException(inner) => inner.fmt(f),
            Error::ConflictErrorException(inner) => inner.fmt(f),
            Error::HomeRegionNotSetException(inner) => inner.fmt(f),
            Error::InvalidParameterException(inner) => inner.fmt(f),
            Error::InvalidParameterValueException(inner) => inner.fmt(f),
            Error::OperationNotPermittedException(inner) => inner.fmt(f),
            Error::ResourceInUseException(inner) => inner.fmt(f),
            Error::ResourceNotFoundException(inner) => inner.fmt(f),
            Error::ServerInternalErrorException(inner) => inner.fmt(f),
            Error::Unhandled(inner) => inner.fmt(f),
        }
    }
}
impl
    From<smithy_http::result::SdkError<crate::error::AssociateConfigurationItemsToApplicationError>>
    for Error
{
    fn from(
        err: smithy_http::result::SdkError<
            crate::error::AssociateConfigurationItemsToApplicationError,
        >,
    ) -> Self {
        match err {
            smithy_http::result::SdkError::ServiceError { err, ..} => match err.kind {
                crate::error::AssociateConfigurationItemsToApplicationErrorKind::AuthorizationErrorException(inner) => Error::AuthorizationErrorException(inner),
                crate::error::AssociateConfigurationItemsToApplicationErrorKind::HomeRegionNotSetException(inner) => Error::HomeRegionNotSetException(inner),
                crate::error::AssociateConfigurationItemsToApplicationErrorKind::InvalidParameterException(inner) => Error::InvalidParameterException(inner),
                crate::error::AssociateConfigurationItemsToApplicationErrorKind::InvalidParameterValueException(inner) => Error::InvalidParameterValueException(inner),
                crate::error::AssociateConfigurationItemsToApplicationErrorKind::ServerInternalErrorException(inner) => Error::ServerInternalErrorException(inner),
                crate::error::AssociateConfigurationItemsToApplicationErrorKind::Unhandled(inner) => Error::Unhandled(inner),
            }
            _ => Error::Unhandled(err.into()),
        }
    }
}
impl From<smithy_http::result::SdkError<crate::error::BatchDeleteImportDataError>> for Error {
    fn from(err: smithy_http::result::SdkError<crate::error::BatchDeleteImportDataError>) -> Self {
        match err {
            smithy_http::result::SdkError::ServiceError { err, .. } => match err.kind {
                crate::error::BatchDeleteImportDataErrorKind::AuthorizationErrorException(
                    inner,
                ) => Error::AuthorizationErrorException(inner),
                crate::error::BatchDeleteImportDataErrorKind::HomeRegionNotSetException(inner) => {
                    Error::HomeRegionNotSetException(inner)
                }
                crate::error::BatchDeleteImportDataErrorKind::InvalidParameterException(inner) => {
                    Error::InvalidParameterException(inner)
                }
                crate::error::BatchDeleteImportDataErrorKind::InvalidParameterValueException(
                    inner,
                ) => Error::InvalidParameterValueException(inner),
                crate::error::BatchDeleteImportDataErrorKind::ServerInternalErrorException(
                    inner,
                ) => Error::ServerInternalErrorException(inner),
                crate::error::BatchDeleteImportDataErrorKind::Unhandled(inner) => {
                    Error::Unhandled(inner)
                }
            },
            _ => Error::Unhandled(err.into()),
        }
    }
}
impl From<smithy_http::result::SdkError<crate::error::CreateApplicationError>> for Error {
    fn from(err: smithy_http::result::SdkError<crate::error::CreateApplicationError>) -> Self {
        match err {
            smithy_http::result::SdkError::ServiceError { err, .. } => match err.kind {
                crate::error::CreateApplicationErrorKind::AuthorizationErrorException(inner) => {
                    Error::AuthorizationErrorException(inner)
                }
                crate::error::CreateApplicationErrorKind::HomeRegionNotSetException(inner) => {
                    Error::HomeRegionNotSetException(inner)
                }
                crate::error::CreateApplicationErrorKind::InvalidParameterException(inner) => {
                    Error::InvalidParameterException(inner)
                }
                crate::error::CreateApplicationErrorKind::InvalidParameterValueException(inner) => {
                    Error::InvalidParameterValueException(inner)
                }
                crate::error::CreateApplicationErrorKind::ServerInternalErrorException(inner) => {
                    Error::ServerInternalErrorException(inner)
                }
                crate::error::CreateApplicationErrorKind::Unhandled(inner) => {
                    Error::Unhandled(inner)
                }
            },
            _ => Error::Unhandled(err.into()),
        }
    }
}
impl From<smithy_http::result::SdkError<crate::error::CreateTagsError>> for Error {
    fn from(err: smithy_http::result::SdkError<crate::error::CreateTagsError>) -> Self {
        match err {
            smithy_http::result::SdkError::ServiceError { err, .. } => match err.kind {
                crate::error::CreateTagsErrorKind::AuthorizationErrorException(inner) => {
                    Error::AuthorizationErrorException(inner)
                }
                crate::error::CreateTagsErrorKind::HomeRegionNotSetException(inner) => {
                    Error::HomeRegionNotSetException(inner)
                }
                crate::error::CreateTagsErrorKind::InvalidParameterException(inner) => {
                    Error::InvalidParameterException(inner)
                }
                crate::error::CreateTagsErrorKind::InvalidParameterValueException(inner) => {
                    Error::InvalidParameterValueException(inner)
                }
                crate::error::CreateTagsErrorKind::ResourceNotFoundException(inner) => {
                    Error::ResourceNotFoundException(inner)
                }
                crate::error::CreateTagsErrorKind::ServerInternalErrorException(inner) => {
                    Error::ServerInternalErrorException(inner)
                }
                crate::error::CreateTagsErrorKind::Unhandled(inner) => Error::Unhandled(inner),
            },
            _ => Error::Unhandled(err.into()),
        }
    }
}
impl From<smithy_http::result::SdkError<crate::error::DeleteApplicationsError>> for Error {
    fn from(err: smithy_http::result::SdkError<crate::error::DeleteApplicationsError>) -> Self {
        match err {
            smithy_http::result::SdkError::ServiceError { err, .. } => match err.kind {
                crate::error::DeleteApplicationsErrorKind::AuthorizationErrorException(inner) => {
                    Error::AuthorizationErrorException(inner)
                }
                crate::error::DeleteApplicationsErrorKind::HomeRegionNotSetException(inner) => {
                    Error::HomeRegionNotSetException(inner)
                }
                crate::error::DeleteApplicationsErrorKind::InvalidParameterException(inner) => {
                    Error::InvalidParameterException(inner)
                }
                crate::error::DeleteApplicationsErrorKind::InvalidParameterValueException(
                    inner,
                ) => Error::InvalidParameterValueException(inner),
                crate::error::DeleteApplicationsErrorKind::ServerInternalErrorException(inner) => {
                    Error::ServerInternalErrorException(inner)
                }
                crate::error::DeleteApplicationsErrorKind::Unhandled(inner) => {
                    Error::Unhandled(inner)
                }
            },
            _ => Error::Unhandled(err.into()),
        }
    }
}
impl From<smithy_http::result::SdkError<crate::error::DeleteTagsError>> for Error {
    fn from(err: smithy_http::result::SdkError<crate::error::DeleteTagsError>) -> Self {
        match err {
            smithy_http::result::SdkError::ServiceError { err, .. } => match err.kind {
                crate::error::DeleteTagsErrorKind::AuthorizationErrorException(inner) => {
                    Error::AuthorizationErrorException(inner)
                }
                crate::error::DeleteTagsErrorKind::HomeRegionNotSetException(inner) => {
                    Error::HomeRegionNotSetException(inner)
                }
                crate::error::DeleteTagsErrorKind::InvalidParameterException(inner) => {
                    Error::InvalidParameterException(inner)
                }
                crate::error::DeleteTagsErrorKind::InvalidParameterValueException(inner) => {
                    Error::InvalidParameterValueException(inner)
                }
                crate::error::DeleteTagsErrorKind::ResourceNotFoundException(inner) => {
                    Error::ResourceNotFoundException(inner)
                }
                crate::error::DeleteTagsErrorKind::ServerInternalErrorException(inner) => {
                    Error::ServerInternalErrorException(inner)
                }
                crate::error::DeleteTagsErrorKind::Unhandled(inner) => Error::Unhandled(inner),
            },
            _ => Error::Unhandled(err.into()),
        }
    }
}
impl From<smithy_http::result::SdkError<crate::error::DescribeAgentsError>> for Error {
    fn from(err: smithy_http::result::SdkError<crate::error::DescribeAgentsError>) -> Self {
        match err {
            smithy_http::result::SdkError::ServiceError { err, .. } => match err.kind {
                crate::error::DescribeAgentsErrorKind::AuthorizationErrorException(inner) => {
                    Error::AuthorizationErrorException(inner)
                }
                crate::error::DescribeAgentsErrorKind::HomeRegionNotSetException(inner) => {
                    Error::HomeRegionNotSetException(inner)
                }
                crate::error::DescribeAgentsErrorKind::InvalidParameterException(inner) => {
                    Error::InvalidParameterException(inner)
                }
                crate::error::DescribeAgentsErrorKind::InvalidParameterValueException(inner) => {
                    Error::InvalidParameterValueException(inner)
                }
                crate::error::DescribeAgentsErrorKind::ServerInternalErrorException(inner) => {
                    Error::ServerInternalErrorException(inner)
                }
                crate::error::DescribeAgentsErrorKind::Unhandled(inner) => Error::Unhandled(inner),
            },
            _ => Error::Unhandled(err.into()),
        }
    }
}
impl From<smithy_http::result::SdkError<crate::error::DescribeConfigurationsError>> for Error {
    fn from(err: smithy_http::result::SdkError<crate::error::DescribeConfigurationsError>) -> Self {
        match err {
            smithy_http::result::SdkError::ServiceError { err, .. } => match err.kind {
                crate::error::DescribeConfigurationsErrorKind::AuthorizationErrorException(
                    inner,
                ) => Error::AuthorizationErrorException(inner),
                crate::error::DescribeConfigurationsErrorKind::HomeRegionNotSetException(inner) => {
                    Error::HomeRegionNotSetException(inner)
                }
                crate::error::DescribeConfigurationsErrorKind::InvalidParameterException(inner) => {
                    Error::InvalidParameterException(inner)
                }
                crate::error::DescribeConfigurationsErrorKind::InvalidParameterValueException(
                    inner,
                ) => Error::InvalidParameterValueException(inner),
                crate::error::DescribeConfigurationsErrorKind::ServerInternalErrorException(
                    inner,
                ) => Error::ServerInternalErrorException(inner),
                crate::error::DescribeConfigurationsErrorKind::Unhandled(inner) => {
                    Error::Unhandled(inner)
                }
            },
            _ => Error::Unhandled(err.into()),
        }
    }
}
impl From<smithy_http::result::SdkError<crate::error::DescribeContinuousExportsError>> for Error {
    fn from(
        err: smithy_http::result::SdkError<crate::error::DescribeContinuousExportsError>,
    ) -> Self {
        match err {
            smithy_http::result::SdkError::ServiceError { err, ..} => match err.kind {
                crate::error::DescribeContinuousExportsErrorKind::AuthorizationErrorException(inner) => Error::AuthorizationErrorException(inner),
                crate::error::DescribeContinuousExportsErrorKind::HomeRegionNotSetException(inner) => Error::HomeRegionNotSetException(inner),
                crate::error::DescribeContinuousExportsErrorKind::InvalidParameterException(inner) => Error::InvalidParameterException(inner),
                crate::error::DescribeContinuousExportsErrorKind::InvalidParameterValueException(inner) => Error::InvalidParameterValueException(inner),
                crate::error::DescribeContinuousExportsErrorKind::OperationNotPermittedException(inner) => Error::OperationNotPermittedException(inner),
                crate::error::DescribeContinuousExportsErrorKind::ResourceNotFoundException(inner) => Error::ResourceNotFoundException(inner),
                crate::error::DescribeContinuousExportsErrorKind::ServerInternalErrorException(inner) => Error::ServerInternalErrorException(inner),
                crate::error::DescribeContinuousExportsErrorKind::Unhandled(inner) => Error::Unhandled(inner),
            }
            _ => Error::Unhandled(err.into()),
        }
    }
}
impl From<smithy_http::result::SdkError<crate::error::DescribeExportConfigurationsError>>
    for Error
{
    fn from(
        err: smithy_http::result::SdkError<crate::error::DescribeExportConfigurationsError>,
    ) -> Self {
        match err {
            smithy_http::result::SdkError::ServiceError { err, ..} => match err.kind {
                crate::error::DescribeExportConfigurationsErrorKind::AuthorizationErrorException(inner) => Error::AuthorizationErrorException(inner),
                crate::error::DescribeExportConfigurationsErrorKind::HomeRegionNotSetException(inner) => Error::HomeRegionNotSetException(inner),
                crate::error::DescribeExportConfigurationsErrorKind::InvalidParameterException(inner) => Error::InvalidParameterException(inner),
                crate::error::DescribeExportConfigurationsErrorKind::InvalidParameterValueException(inner) => Error::InvalidParameterValueException(inner),
                crate::error::DescribeExportConfigurationsErrorKind::ResourceNotFoundException(inner) => Error::ResourceNotFoundException(inner),
                crate::error::DescribeExportConfigurationsErrorKind::ServerInternalErrorException(inner) => Error::ServerInternalErrorException(inner),
                crate::error::DescribeExportConfigurationsErrorKind::Unhandled(inner) => Error::Unhandled(inner),
            }
            _ => Error::Unhandled(err.into()),
        }
    }
}
impl From<smithy_http::result::SdkError<crate::error::DescribeExportTasksError>> for Error {
    fn from(err: smithy_http::result::SdkError<crate::error::DescribeExportTasksError>) -> Self {
        match err {
            smithy_http::result::SdkError::ServiceError { err, .. } => match err.kind {
                crate::error::DescribeExportTasksErrorKind::AuthorizationErrorException(inner) => {
                    Error::AuthorizationErrorException(inner)
                }
                crate::error::DescribeExportTasksErrorKind::HomeRegionNotSetException(inner) => {
                    Error::HomeRegionNotSetException(inner)
                }
                crate::error::DescribeExportTasksErrorKind::InvalidParameterException(inner) => {
                    Error::InvalidParameterException(inner)
                }
                crate::error::DescribeExportTasksErrorKind::InvalidParameterValueException(
                    inner,
                ) => Error::InvalidParameterValueException(inner),
                crate::error::DescribeExportTasksErrorKind::ServerInternalErrorException(inner) => {
                    Error::ServerInternalErrorException(inner)
                }
                crate::error::DescribeExportTasksErrorKind::Unhandled(inner) => {
                    Error::Unhandled(inner)
                }
            },
            _ => Error::Unhandled(err.into()),
        }
    }
}
impl From<smithy_http::result::SdkError<crate::error::DescribeImportTasksError>> for Error {
    fn from(err: smithy_http::result::SdkError<crate::error::DescribeImportTasksError>) -> Self {
        match err {
            smithy_http::result::SdkError::ServiceError { err, .. } => match err.kind {
                crate::error::DescribeImportTasksErrorKind::AuthorizationErrorException(inner) => {
                    Error::AuthorizationErrorException(inner)
                }
                crate::error::DescribeImportTasksErrorKind::HomeRegionNotSetException(inner) => {
                    Error::HomeRegionNotSetException(inner)
                }
                crate::error::DescribeImportTasksErrorKind::InvalidParameterException(inner) => {
                    Error::InvalidParameterException(inner)
                }
                crate::error::DescribeImportTasksErrorKind::InvalidParameterValueException(
                    inner,
                ) => Error::InvalidParameterValueException(inner),
                crate::error::DescribeImportTasksErrorKind::ServerInternalErrorException(inner) => {
                    Error::ServerInternalErrorException(inner)
                }
                crate::error::DescribeImportTasksErrorKind::Unhandled(inner) => {
                    Error::Unhandled(inner)
                }
            },
            _ => Error::Unhandled(err.into()),
        }
    }
}
impl From<smithy_http::result::SdkError<crate::error::DescribeTagsError>> for Error {
    fn from(err: smithy_http::result::SdkError<crate::error::DescribeTagsError>) -> Self {
        match err {
            smithy_http::result::SdkError::ServiceError { err, .. } => match err.kind {
                crate::error::DescribeTagsErrorKind::AuthorizationErrorException(inner) => {
                    Error::AuthorizationErrorException(inner)
                }
                crate::error::DescribeTagsErrorKind::HomeRegionNotSetException(inner) => {
                    Error::HomeRegionNotSetException(inner)
                }
                crate::error::DescribeTagsErrorKind::InvalidParameterException(inner) => {
                    Error::InvalidParameterException(inner)
                }
                crate::error::DescribeTagsErrorKind::InvalidParameterValueException(inner) => {
                    Error::InvalidParameterValueException(inner)
                }
                crate::error::DescribeTagsErrorKind::ResourceNotFoundException(inner) => {
                    Error::ResourceNotFoundException(inner)
                }
                crate::error::DescribeTagsErrorKind::ServerInternalErrorException(inner) => {
                    Error::ServerInternalErrorException(inner)
                }
                crate::error::DescribeTagsErrorKind::Unhandled(inner) => Error::Unhandled(inner),
            },
            _ => Error::Unhandled(err.into()),
        }
    }
}
impl
    From<
        smithy_http::result::SdkError<
            crate::error::DisassociateConfigurationItemsFromApplicationError,
        >,
    > for Error
{
    fn from(
        err: smithy_http::result::SdkError<
            crate::error::DisassociateConfigurationItemsFromApplicationError,
        >,
    ) -> Self {
        match err {
            smithy_http::result::SdkError::ServiceError { err, ..} => match err.kind {
                crate::error::DisassociateConfigurationItemsFromApplicationErrorKind::AuthorizationErrorException(inner) => Error::AuthorizationErrorException(inner),
                crate::error::DisassociateConfigurationItemsFromApplicationErrorKind::HomeRegionNotSetException(inner) => Error::HomeRegionNotSetException(inner),
                crate::error::DisassociateConfigurationItemsFromApplicationErrorKind::InvalidParameterException(inner) => Error::InvalidParameterException(inner),
                crate::error::DisassociateConfigurationItemsFromApplicationErrorKind::InvalidParameterValueException(inner) => Error::InvalidParameterValueException(inner),
                crate::error::DisassociateConfigurationItemsFromApplicationErrorKind::ServerInternalErrorException(inner) => Error::ServerInternalErrorException(inner),
                crate::error::DisassociateConfigurationItemsFromApplicationErrorKind::Unhandled(inner) => Error::Unhandled(inner),
            }
            _ => Error::Unhandled(err.into()),
        }
    }
}
impl From<smithy_http::result::SdkError<crate::error::ExportConfigurationsError>> for Error {
    fn from(err: smithy_http::result::SdkError<crate::error::ExportConfigurationsError>) -> Self {
        match err {
            smithy_http::result::SdkError::ServiceError { err, .. } => match err.kind {
                crate::error::ExportConfigurationsErrorKind::AuthorizationErrorException(inner) => {
                    Error::AuthorizationErrorException(inner)
                }
                crate::error::ExportConfigurationsErrorKind::HomeRegionNotSetException(inner) => {
                    Error::HomeRegionNotSetException(inner)
                }
                crate::error::ExportConfigurationsErrorKind::InvalidParameterException(inner) => {
                    Error::InvalidParameterException(inner)
                }
                crate::error::ExportConfigurationsErrorKind::InvalidParameterValueException(
                    inner,
                ) => Error::InvalidParameterValueException(inner),
                crate::error::ExportConfigurationsErrorKind::OperationNotPermittedException(
                    inner,
                ) => Error::OperationNotPermittedException(inner),
                crate::error::ExportConfigurationsErrorKind::ServerInternalErrorException(
                    inner,
                ) => Error::ServerInternalErrorException(inner),
                crate::error::ExportConfigurationsErrorKind::Unhandled(inner) => {
                    Error::Unhandled(inner)
                }
            },
            _ => Error::Unhandled(err.into()),
        }
    }
}
impl From<smithy_http::result::SdkError<crate::error::GetDiscoverySummaryError>> for Error {
    fn from(err: smithy_http::result::SdkError<crate::error::GetDiscoverySummaryError>) -> Self {
        match err {
            smithy_http::result::SdkError::ServiceError { err, .. } => match err.kind {
                crate::error::GetDiscoverySummaryErrorKind::AuthorizationErrorException(inner) => {
                    Error::AuthorizationErrorException(inner)
                }
                crate::error::GetDiscoverySummaryErrorKind::HomeRegionNotSetException(inner) => {
                    Error::HomeRegionNotSetException(inner)
                }
                crate::error::GetDiscoverySummaryErrorKind::InvalidParameterException(inner) => {
                    Error::InvalidParameterException(inner)
                }
                crate::error::GetDiscoverySummaryErrorKind::InvalidParameterValueException(
                    inner,
                ) => Error::InvalidParameterValueException(inner),
                crate::error::GetDiscoverySummaryErrorKind::ServerInternalErrorException(inner) => {
                    Error::ServerInternalErrorException(inner)
                }
                crate::error::GetDiscoverySummaryErrorKind::Unhandled(inner) => {
                    Error::Unhandled(inner)
                }
            },
            _ => Error::Unhandled(err.into()),
        }
    }
}
impl From<smithy_http::result::SdkError<crate::error::ListConfigurationsError>> for Error {
    fn from(err: smithy_http::result::SdkError<crate::error::ListConfigurationsError>) -> Self {
        match err {
            smithy_http::result::SdkError::ServiceError { err, .. } => match err.kind {
                crate::error::ListConfigurationsErrorKind::AuthorizationErrorException(inner) => {
                    Error::AuthorizationErrorException(inner)
                }
                crate::error::ListConfigurationsErrorKind::HomeRegionNotSetException(inner) => {
                    Error::HomeRegionNotSetException(inner)
                }
                crate::error::ListConfigurationsErrorKind::InvalidParameterException(inner) => {
                    Error::InvalidParameterException(inner)
                }
                crate::error::ListConfigurationsErrorKind::InvalidParameterValueException(
                    inner,
                ) => Error::InvalidParameterValueException(inner),
                crate::error::ListConfigurationsErrorKind::ResourceNotFoundException(inner) => {
                    Error::ResourceNotFoundException(inner)
                }
                crate::error::ListConfigurationsErrorKind::ServerInternalErrorException(inner) => {
                    Error::ServerInternalErrorException(inner)
                }
                crate::error::ListConfigurationsErrorKind::Unhandled(inner) => {
                    Error::Unhandled(inner)
                }
            },
            _ => Error::Unhandled(err.into()),
        }
    }
}
impl From<smithy_http::result::SdkError<crate::error::ListServerNeighborsError>> for Error {
    fn from(err: smithy_http::result::SdkError<crate::error::ListServerNeighborsError>) -> Self {
        match err {
            smithy_http::result::SdkError::ServiceError { err, .. } => match err.kind {
                crate::error::ListServerNeighborsErrorKind::AuthorizationErrorException(inner) => {
                    Error::AuthorizationErrorException(inner)
                }
                crate::error::ListServerNeighborsErrorKind::HomeRegionNotSetException(inner) => {
                    Error::HomeRegionNotSetException(inner)
                }
                crate::error::ListServerNeighborsErrorKind::InvalidParameterException(inner) => {
                    Error::InvalidParameterException(inner)
                }
                crate::error::ListServerNeighborsErrorKind::InvalidParameterValueException(
                    inner,
                ) => Error::InvalidParameterValueException(inner),
                crate::error::ListServerNeighborsErrorKind::ServerInternalErrorException(inner) => {
                    Error::ServerInternalErrorException(inner)
                }
                crate::error::ListServerNeighborsErrorKind::Unhandled(inner) => {
                    Error::Unhandled(inner)
                }
            },
            _ => Error::Unhandled(err.into()),
        }
    }
}
impl From<smithy_http::result::SdkError<crate::error::StartContinuousExportError>> for Error {
    fn from(err: smithy_http::result::SdkError<crate::error::StartContinuousExportError>) -> Self {
        match err {
            smithy_http::result::SdkError::ServiceError { err, .. } => match err.kind {
                crate::error::StartContinuousExportErrorKind::AuthorizationErrorException(
                    inner,
                ) => Error::AuthorizationErrorException(inner),
                crate::error::StartContinuousExportErrorKind::ConflictErrorException(inner) => {
                    Error::ConflictErrorException(inner)
                }
                crate::error::StartContinuousExportErrorKind::HomeRegionNotSetException(inner) => {
                    Error::HomeRegionNotSetException(inner)
                }
                crate::error::StartContinuousExportErrorKind::InvalidParameterException(inner) => {
                    Error::InvalidParameterException(inner)
                }
                crate::error::StartContinuousExportErrorKind::InvalidParameterValueException(
                    inner,
                ) => Error::InvalidParameterValueException(inner),
                crate::error::StartContinuousExportErrorKind::OperationNotPermittedException(
                    inner,
                ) => Error::OperationNotPermittedException(inner),
                crate::error::StartContinuousExportErrorKind::ResourceInUseException(inner) => {
                    Error::ResourceInUseException(inner)
                }
                crate::error::StartContinuousExportErrorKind::ServerInternalErrorException(
                    inner,
                ) => Error::ServerInternalErrorException(inner),
                crate::error::StartContinuousExportErrorKind::Unhandled(inner) => {
                    Error::Unhandled(inner)
                }
            },
            _ => Error::Unhandled(err.into()),
        }
    }
}
impl From<smithy_http::result::SdkError<crate::error::StartDataCollectionByAgentIdsError>>
    for Error
{
    fn from(
        err: smithy_http::result::SdkError<crate::error::StartDataCollectionByAgentIdsError>,
    ) -> Self {
        match err {
            smithy_http::result::SdkError::ServiceError { err, ..} => match err.kind {
                crate::error::StartDataCollectionByAgentIdsErrorKind::AuthorizationErrorException(inner) => Error::AuthorizationErrorException(inner),
                crate::error::StartDataCollectionByAgentIdsErrorKind::HomeRegionNotSetException(inner) => Error::HomeRegionNotSetException(inner),
                crate::error::StartDataCollectionByAgentIdsErrorKind::InvalidParameterException(inner) => Error::InvalidParameterException(inner),
                crate::error::StartDataCollectionByAgentIdsErrorKind::InvalidParameterValueException(inner) => Error::InvalidParameterValueException(inner),
                crate::error::StartDataCollectionByAgentIdsErrorKind::ServerInternalErrorException(inner) => Error::ServerInternalErrorException(inner),
                crate::error::StartDataCollectionByAgentIdsErrorKind::Unhandled(inner) => Error::Unhandled(inner),
            }
            _ => Error::Unhandled(err.into()),
        }
    }
}
impl From<smithy_http::result::SdkError<crate::error::StartExportTaskError>> for Error {
    fn from(err: smithy_http::result::SdkError<crate::error::StartExportTaskError>) -> Self {
        match err {
            smithy_http::result::SdkError::ServiceError { err, .. } => match err.kind {
                crate::error::StartExportTaskErrorKind::AuthorizationErrorException(inner) => {
                    Error::AuthorizationErrorException(inner)
                }
                crate::error::StartExportTaskErrorKind::HomeRegionNotSetException(inner) => {
                    Error::HomeRegionNotSetException(inner)
                }
                crate::error::StartExportTaskErrorKind::InvalidParameterException(inner) => {
                    Error::InvalidParameterException(inner)
                }
                crate::error::StartExportTaskErrorKind::InvalidParameterValueException(inner) => {
                    Error::InvalidParameterValueException(inner)
                }
                crate::error::StartExportTaskErrorKind::OperationNotPermittedException(inner) => {
                    Error::OperationNotPermittedException(inner)
                }
                crate::error::StartExportTaskErrorKind::ServerInternalErrorException(inner) => {
                    Error::ServerInternalErrorException(inner)
                }
                crate::error::StartExportTaskErrorKind::Unhandled(inner) => Error::Unhandled(inner),
            },
            _ => Error::Unhandled(err.into()),
        }
    }
}
impl From<smithy_http::result::SdkError<crate::error::StartImportTaskError>> for Error {
    fn from(err: smithy_http::result::SdkError<crate::error::StartImportTaskError>) -> Self {
        match err {
            smithy_http::result::SdkError::ServiceError { err, .. } => match err.kind {
                crate::error::StartImportTaskErrorKind::AuthorizationErrorException(inner) => {
                    Error::AuthorizationErrorException(inner)
                }
                crate::error::StartImportTaskErrorKind::HomeRegionNotSetException(inner) => {
                    Error::HomeRegionNotSetException(inner)
                }
                crate::error::StartImportTaskErrorKind::InvalidParameterException(inner) => {
                    Error::InvalidParameterException(inner)
                }
                crate::error::StartImportTaskErrorKind::InvalidParameterValueException(inner) => {
                    Error::InvalidParameterValueException(inner)
                }
                crate::error::StartImportTaskErrorKind::ResourceInUseException(inner) => {
                    Error::ResourceInUseException(inner)
                }
                crate::error::StartImportTaskErrorKind::ServerInternalErrorException(inner) => {
                    Error::ServerInternalErrorException(inner)
                }
                crate::error::StartImportTaskErrorKind::Unhandled(inner) => Error::Unhandled(inner),
            },
            _ => Error::Unhandled(err.into()),
        }
    }
}
impl From<smithy_http::result::SdkError<crate::error::StopContinuousExportError>> for Error {
    fn from(err: smithy_http::result::SdkError<crate::error::StopContinuousExportError>) -> Self {
        match err {
            smithy_http::result::SdkError::ServiceError { err, .. } => match err.kind {
                crate::error::StopContinuousExportErrorKind::AuthorizationErrorException(inner) => {
                    Error::AuthorizationErrorException(inner)
                }
                crate::error::StopContinuousExportErrorKind::HomeRegionNotSetException(inner) => {
                    Error::HomeRegionNotSetException(inner)
                }
                crate::error::StopContinuousExportErrorKind::InvalidParameterException(inner) => {
                    Error::InvalidParameterException(inner)
                }
                crate::error::StopContinuousExportErrorKind::InvalidParameterValueException(
                    inner,
                ) => Error::InvalidParameterValueException(inner),
                crate::error::StopContinuousExportErrorKind::OperationNotPermittedException(
                    inner,
                ) => Error::OperationNotPermittedException(inner),
                crate::error::StopContinuousExportErrorKind::ResourceInUseException(inner) => {
                    Error::ResourceInUseException(inner)
                }
                crate::error::StopContinuousExportErrorKind::ResourceNotFoundException(inner) => {
                    Error::ResourceNotFoundException(inner)
                }
                crate::error::StopContinuousExportErrorKind::ServerInternalErrorException(
                    inner,
                ) => Error::ServerInternalErrorException(inner),
                crate::error::StopContinuousExportErrorKind::Unhandled(inner) => {
                    Error::Unhandled(inner)
                }
            },
            _ => Error::Unhandled(err.into()),
        }
    }
}
impl From<smithy_http::result::SdkError<crate::error::StopDataCollectionByAgentIdsError>>
    for Error
{
    fn from(
        err: smithy_http::result::SdkError<crate::error::StopDataCollectionByAgentIdsError>,
    ) -> Self {
        match err {
            smithy_http::result::SdkError::ServiceError { err, ..} => match err.kind {
                crate::error::StopDataCollectionByAgentIdsErrorKind::AuthorizationErrorException(inner) => Error::AuthorizationErrorException(inner),
                crate::error::StopDataCollectionByAgentIdsErrorKind::HomeRegionNotSetException(inner) => Error::HomeRegionNotSetException(inner),
                crate::error::StopDataCollectionByAgentIdsErrorKind::InvalidParameterException(inner) => Error::InvalidParameterException(inner),
                crate::error::StopDataCollectionByAgentIdsErrorKind::InvalidParameterValueException(inner) => Error::InvalidParameterValueException(inner),
                crate::error::StopDataCollectionByAgentIdsErrorKind::ServerInternalErrorException(inner) => Error::ServerInternalErrorException(inner),
                crate::error::StopDataCollectionByAgentIdsErrorKind::Unhandled(inner) => Error::Unhandled(inner),
            }
            _ => Error::Unhandled(err.into()),
        }
    }
}
impl From<smithy_http::result::SdkError<crate::error::UpdateApplicationError>> for Error {
    fn from(err: smithy_http::result::SdkError<crate::error::UpdateApplicationError>) -> Self {
        match err {
            smithy_http::result::SdkError::ServiceError { err, .. } => match err.kind {
                crate::error::UpdateApplicationErrorKind::AuthorizationErrorException(inner) => {
                    Error::AuthorizationErrorException(inner)
                }
                crate::error::UpdateApplicationErrorKind::HomeRegionNotSetException(inner) => {
                    Error::HomeRegionNotSetException(inner)
                }
                crate::error::UpdateApplicationErrorKind::InvalidParameterException(inner) => {
                    Error::InvalidParameterException(inner)
                }
                crate::error::UpdateApplicationErrorKind::InvalidParameterValueException(inner) => {
                    Error::InvalidParameterValueException(inner)
                }
                crate::error::UpdateApplicationErrorKind::ServerInternalErrorException(inner) => {
                    Error::ServerInternalErrorException(inner)
                }
                crate::error::UpdateApplicationErrorKind::Unhandled(inner) => {
                    Error::Unhandled(inner)
                }
            },
            _ => Error::Unhandled(err.into()),
        }
    }
}
impl std::error::Error for Error {}