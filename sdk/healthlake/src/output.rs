// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
#[non_exhaustive]
#[derive(std::clone::Clone, std::cmp::PartialEq)]
pub struct StartFhirImportJobOutput {
    /// <p>The AWS-generated job ID.</p>
    pub job_id: std::option::Option<std::string::String>,
    /// <p>The status of an import job.</p>
    pub job_status: std::option::Option<crate::model::JobStatus>,
    /// <p>The AWS-generated Data Store ID.</p>
    pub datastore_id: std::option::Option<std::string::String>,
}
impl std::fmt::Debug for StartFhirImportJobOutput {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut formatter = f.debug_struct("StartFhirImportJobOutput");
        formatter.field("job_id", &self.job_id);
        formatter.field("job_status", &self.job_status);
        formatter.field("datastore_id", &self.datastore_id);
        formatter.finish()
    }
}
/// See [`StartFhirImportJobOutput`](crate::output::StartFhirImportJobOutput)
pub mod start_fhir_import_job_output {
    /// A builder for [`StartFhirImportJobOutput`](crate::output::StartFhirImportJobOutput)
    #[non_exhaustive]
    #[derive(std::default::Default, std::clone::Clone, std::cmp::PartialEq, std::fmt::Debug)]
    pub struct Builder {
        pub(crate) job_id: std::option::Option<std::string::String>,
        pub(crate) job_status: std::option::Option<crate::model::JobStatus>,
        pub(crate) datastore_id: std::option::Option<std::string::String>,
    }
    impl Builder {
        /// <p>The AWS-generated job ID.</p>
        pub fn job_id(mut self, input: impl Into<std::string::String>) -> Self {
            self.job_id = Some(input.into());
            self
        }
        pub fn set_job_id(mut self, input: std::option::Option<std::string::String>) -> Self {
            self.job_id = input;
            self
        }
        /// <p>The status of an import job.</p>
        pub fn job_status(mut self, input: crate::model::JobStatus) -> Self {
            self.job_status = Some(input);
            self
        }
        pub fn set_job_status(
            mut self,
            input: std::option::Option<crate::model::JobStatus>,
        ) -> Self {
            self.job_status = input;
            self
        }
        /// <p>The AWS-generated Data Store ID.</p>
        pub fn datastore_id(mut self, input: impl Into<std::string::String>) -> Self {
            self.datastore_id = Some(input.into());
            self
        }
        pub fn set_datastore_id(mut self, input: std::option::Option<std::string::String>) -> Self {
            self.datastore_id = input;
            self
        }
        /// Consumes the builder and constructs a [`StartFhirImportJobOutput`](crate::output::StartFhirImportJobOutput)
        pub fn build(self) -> crate::output::StartFhirImportJobOutput {
            crate::output::StartFhirImportJobOutput {
                job_id: self.job_id,
                job_status: self.job_status,
                datastore_id: self.datastore_id,
            }
        }
    }
}
impl StartFhirImportJobOutput {
    /// Creates a new builder-style object to manufacture [`StartFhirImportJobOutput`](crate::output::StartFhirImportJobOutput)
    pub fn builder() -> crate::output::start_fhir_import_job_output::Builder {
        crate::output::start_fhir_import_job_output::Builder::default()
    }
}

#[non_exhaustive]
#[derive(std::clone::Clone, std::cmp::PartialEq)]
pub struct StartFhirExportJobOutput {
    /// <p>The AWS generated ID for an export job.</p>
    pub job_id: std::option::Option<std::string::String>,
    /// <p>The status of a FHIR export job. Possible statuses are SUBMITTED, IN_PROGRESS, COMPLETED, or FAILED.</p>
    pub job_status: std::option::Option<crate::model::JobStatus>,
    /// <p>The AWS generated ID for the Data Store from which files are being exported for an export job.</p>
    pub datastore_id: std::option::Option<std::string::String>,
}
impl std::fmt::Debug for StartFhirExportJobOutput {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut formatter = f.debug_struct("StartFhirExportJobOutput");
        formatter.field("job_id", &self.job_id);
        formatter.field("job_status", &self.job_status);
        formatter.field("datastore_id", &self.datastore_id);
        formatter.finish()
    }
}
/// See [`StartFhirExportJobOutput`](crate::output::StartFhirExportJobOutput)
pub mod start_fhir_export_job_output {
    /// A builder for [`StartFhirExportJobOutput`](crate::output::StartFhirExportJobOutput)
    #[non_exhaustive]
    #[derive(std::default::Default, std::clone::Clone, std::cmp::PartialEq, std::fmt::Debug)]
    pub struct Builder {
        pub(crate) job_id: std::option::Option<std::string::String>,
        pub(crate) job_status: std::option::Option<crate::model::JobStatus>,
        pub(crate) datastore_id: std::option::Option<std::string::String>,
    }
    impl Builder {
        /// <p>The AWS generated ID for an export job.</p>
        pub fn job_id(mut self, input: impl Into<std::string::String>) -> Self {
            self.job_id = Some(input.into());
            self
        }
        pub fn set_job_id(mut self, input: std::option::Option<std::string::String>) -> Self {
            self.job_id = input;
            self
        }
        /// <p>The status of a FHIR export job. Possible statuses are SUBMITTED, IN_PROGRESS, COMPLETED, or FAILED.</p>
        pub fn job_status(mut self, input: crate::model::JobStatus) -> Self {
            self.job_status = Some(input);
            self
        }
        pub fn set_job_status(
            mut self,
            input: std::option::Option<crate::model::JobStatus>,
        ) -> Self {
            self.job_status = input;
            self
        }
        /// <p>The AWS generated ID for the Data Store from which files are being exported for an export job.</p>
        pub fn datastore_id(mut self, input: impl Into<std::string::String>) -> Self {
            self.datastore_id = Some(input.into());
            self
        }
        pub fn set_datastore_id(mut self, input: std::option::Option<std::string::String>) -> Self {
            self.datastore_id = input;
            self
        }
        /// Consumes the builder and constructs a [`StartFhirExportJobOutput`](crate::output::StartFhirExportJobOutput)
        pub fn build(self) -> crate::output::StartFhirExportJobOutput {
            crate::output::StartFhirExportJobOutput {
                job_id: self.job_id,
                job_status: self.job_status,
                datastore_id: self.datastore_id,
            }
        }
    }
}
impl StartFhirExportJobOutput {
    /// Creates a new builder-style object to manufacture [`StartFhirExportJobOutput`](crate::output::StartFhirExportJobOutput)
    pub fn builder() -> crate::output::start_fhir_export_job_output::Builder {
        crate::output::start_fhir_export_job_output::Builder::default()
    }
}

#[non_exhaustive]
#[derive(std::clone::Clone, std::cmp::PartialEq)]
pub struct ListFhirDatastoresOutput {
    /// <p>All properties associated with the listed Data Stores.</p>
    pub datastore_properties_list:
        std::option::Option<std::vec::Vec<crate::model::DatastoreProperties>>,
    /// <p>Pagination token that can be used to retrieve the next page of results.</p>
    pub next_token: std::option::Option<std::string::String>,
}
impl std::fmt::Debug for ListFhirDatastoresOutput {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut formatter = f.debug_struct("ListFhirDatastoresOutput");
        formatter.field("datastore_properties_list", &self.datastore_properties_list);
        formatter.field("next_token", &self.next_token);
        formatter.finish()
    }
}
/// See [`ListFhirDatastoresOutput`](crate::output::ListFhirDatastoresOutput)
pub mod list_fhir_datastores_output {
    /// A builder for [`ListFhirDatastoresOutput`](crate::output::ListFhirDatastoresOutput)
    #[non_exhaustive]
    #[derive(std::default::Default, std::clone::Clone, std::cmp::PartialEq, std::fmt::Debug)]
    pub struct Builder {
        pub(crate) datastore_properties_list:
            std::option::Option<std::vec::Vec<crate::model::DatastoreProperties>>,
        pub(crate) next_token: std::option::Option<std::string::String>,
    }
    impl Builder {
        pub fn datastore_properties_list(
            mut self,
            input: impl Into<crate::model::DatastoreProperties>,
        ) -> Self {
            let mut v = self.datastore_properties_list.unwrap_or_default();
            v.push(input.into());
            self.datastore_properties_list = Some(v);
            self
        }
        pub fn set_datastore_properties_list(
            mut self,
            input: std::option::Option<std::vec::Vec<crate::model::DatastoreProperties>>,
        ) -> Self {
            self.datastore_properties_list = input;
            self
        }
        /// <p>Pagination token that can be used to retrieve the next page of results.</p>
        pub fn next_token(mut self, input: impl Into<std::string::String>) -> Self {
            self.next_token = Some(input.into());
            self
        }
        pub fn set_next_token(mut self, input: std::option::Option<std::string::String>) -> Self {
            self.next_token = input;
            self
        }
        /// Consumes the builder and constructs a [`ListFhirDatastoresOutput`](crate::output::ListFhirDatastoresOutput)
        pub fn build(self) -> crate::output::ListFhirDatastoresOutput {
            crate::output::ListFhirDatastoresOutput {
                datastore_properties_list: self.datastore_properties_list,
                next_token: self.next_token,
            }
        }
    }
}
impl ListFhirDatastoresOutput {
    /// Creates a new builder-style object to manufacture [`ListFhirDatastoresOutput`](crate::output::ListFhirDatastoresOutput)
    pub fn builder() -> crate::output::list_fhir_datastores_output::Builder {
        crate::output::list_fhir_datastores_output::Builder::default()
    }
}

#[non_exhaustive]
#[derive(std::clone::Clone, std::cmp::PartialEq)]
pub struct DescribeFhirImportJobOutput {
    /// <p>The properties of the Import job request, including the ID, ARN, name, and the status of the job.</p>
    pub import_job_properties: std::option::Option<crate::model::ImportJobProperties>,
}
impl std::fmt::Debug for DescribeFhirImportJobOutput {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut formatter = f.debug_struct("DescribeFhirImportJobOutput");
        formatter.field("import_job_properties", &self.import_job_properties);
        formatter.finish()
    }
}
/// See [`DescribeFhirImportJobOutput`](crate::output::DescribeFhirImportJobOutput)
pub mod describe_fhir_import_job_output {
    /// A builder for [`DescribeFhirImportJobOutput`](crate::output::DescribeFhirImportJobOutput)
    #[non_exhaustive]
    #[derive(std::default::Default, std::clone::Clone, std::cmp::PartialEq, std::fmt::Debug)]
    pub struct Builder {
        pub(crate) import_job_properties: std::option::Option<crate::model::ImportJobProperties>,
    }
    impl Builder {
        /// <p>The properties of the Import job request, including the ID, ARN, name, and the status of the job.</p>
        pub fn import_job_properties(mut self, input: crate::model::ImportJobProperties) -> Self {
            self.import_job_properties = Some(input);
            self
        }
        pub fn set_import_job_properties(
            mut self,
            input: std::option::Option<crate::model::ImportJobProperties>,
        ) -> Self {
            self.import_job_properties = input;
            self
        }
        /// Consumes the builder and constructs a [`DescribeFhirImportJobOutput`](crate::output::DescribeFhirImportJobOutput)
        pub fn build(self) -> crate::output::DescribeFhirImportJobOutput {
            crate::output::DescribeFhirImportJobOutput {
                import_job_properties: self.import_job_properties,
            }
        }
    }
}
impl DescribeFhirImportJobOutput {
    /// Creates a new builder-style object to manufacture [`DescribeFhirImportJobOutput`](crate::output::DescribeFhirImportJobOutput)
    pub fn builder() -> crate::output::describe_fhir_import_job_output::Builder {
        crate::output::describe_fhir_import_job_output::Builder::default()
    }
}

#[non_exhaustive]
#[derive(std::clone::Clone, std::cmp::PartialEq)]
pub struct DescribeFhirExportJobOutput {
    /// <p>Displays the properties of the export job, including the ID, Arn, Name, and the status of the job. </p>
    pub export_job_properties: std::option::Option<crate::model::ExportJobProperties>,
}
impl std::fmt::Debug for DescribeFhirExportJobOutput {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut formatter = f.debug_struct("DescribeFhirExportJobOutput");
        formatter.field("export_job_properties", &self.export_job_properties);
        formatter.finish()
    }
}
/// See [`DescribeFhirExportJobOutput`](crate::output::DescribeFhirExportJobOutput)
pub mod describe_fhir_export_job_output {
    /// A builder for [`DescribeFhirExportJobOutput`](crate::output::DescribeFhirExportJobOutput)
    #[non_exhaustive]
    #[derive(std::default::Default, std::clone::Clone, std::cmp::PartialEq, std::fmt::Debug)]
    pub struct Builder {
        pub(crate) export_job_properties: std::option::Option<crate::model::ExportJobProperties>,
    }
    impl Builder {
        /// <p>Displays the properties of the export job, including the ID, Arn, Name, and the status of the job. </p>
        pub fn export_job_properties(mut self, input: crate::model::ExportJobProperties) -> Self {
            self.export_job_properties = Some(input);
            self
        }
        pub fn set_export_job_properties(
            mut self,
            input: std::option::Option<crate::model::ExportJobProperties>,
        ) -> Self {
            self.export_job_properties = input;
            self
        }
        /// Consumes the builder and constructs a [`DescribeFhirExportJobOutput`](crate::output::DescribeFhirExportJobOutput)
        pub fn build(self) -> crate::output::DescribeFhirExportJobOutput {
            crate::output::DescribeFhirExportJobOutput {
                export_job_properties: self.export_job_properties,
            }
        }
    }
}
impl DescribeFhirExportJobOutput {
    /// Creates a new builder-style object to manufacture [`DescribeFhirExportJobOutput`](crate::output::DescribeFhirExportJobOutput)
    pub fn builder() -> crate::output::describe_fhir_export_job_output::Builder {
        crate::output::describe_fhir_export_job_output::Builder::default()
    }
}

#[non_exhaustive]
#[derive(std::clone::Clone, std::cmp::PartialEq)]
pub struct DescribeFhirDatastoreOutput {
    /// <p>All properties associated with a Data Store, including the Data Store ID, Data Store ARN,
    /// Data Store name, Data Store status, created at, Data Store type version, and Data Store
    /// endpoint.</p>
    pub datastore_properties: std::option::Option<crate::model::DatastoreProperties>,
}
impl std::fmt::Debug for DescribeFhirDatastoreOutput {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut formatter = f.debug_struct("DescribeFhirDatastoreOutput");
        formatter.field("datastore_properties", &self.datastore_properties);
        formatter.finish()
    }
}
/// See [`DescribeFhirDatastoreOutput`](crate::output::DescribeFhirDatastoreOutput)
pub mod describe_fhir_datastore_output {
    /// A builder for [`DescribeFhirDatastoreOutput`](crate::output::DescribeFhirDatastoreOutput)
    #[non_exhaustive]
    #[derive(std::default::Default, std::clone::Clone, std::cmp::PartialEq, std::fmt::Debug)]
    pub struct Builder {
        pub(crate) datastore_properties: std::option::Option<crate::model::DatastoreProperties>,
    }
    impl Builder {
        /// <p>All properties associated with a Data Store, including the Data Store ID, Data Store ARN,
        /// Data Store name, Data Store status, created at, Data Store type version, and Data Store
        /// endpoint.</p>
        pub fn datastore_properties(mut self, input: crate::model::DatastoreProperties) -> Self {
            self.datastore_properties = Some(input);
            self
        }
        pub fn set_datastore_properties(
            mut self,
            input: std::option::Option<crate::model::DatastoreProperties>,
        ) -> Self {
            self.datastore_properties = input;
            self
        }
        /// Consumes the builder and constructs a [`DescribeFhirDatastoreOutput`](crate::output::DescribeFhirDatastoreOutput)
        pub fn build(self) -> crate::output::DescribeFhirDatastoreOutput {
            crate::output::DescribeFhirDatastoreOutput {
                datastore_properties: self.datastore_properties,
            }
        }
    }
}
impl DescribeFhirDatastoreOutput {
    /// Creates a new builder-style object to manufacture [`DescribeFhirDatastoreOutput`](crate::output::DescribeFhirDatastoreOutput)
    pub fn builder() -> crate::output::describe_fhir_datastore_output::Builder {
        crate::output::describe_fhir_datastore_output::Builder::default()
    }
}

#[non_exhaustive]
#[derive(std::clone::Clone, std::cmp::PartialEq)]
pub struct DeleteFhirDatastoreOutput {
    /// <p>The AWS-generated ID for the Data Store to be deleted.</p>
    pub datastore_id: std::option::Option<std::string::String>,
    /// <p>The Amazon Resource Name (ARN) that gives Amazon HealthLake access permission.</p>
    pub datastore_arn: std::option::Option<std::string::String>,
    /// <p>The status of the Data Store that the user has requested to be deleted.
    /// </p>
    pub datastore_status: std::option::Option<crate::model::DatastoreStatus>,
    /// <p>The AWS endpoint for the Data Store the user has requested to be deleted.</p>
    pub datastore_endpoint: std::option::Option<std::string::String>,
}
impl std::fmt::Debug for DeleteFhirDatastoreOutput {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut formatter = f.debug_struct("DeleteFhirDatastoreOutput");
        formatter.field("datastore_id", &self.datastore_id);
        formatter.field("datastore_arn", &self.datastore_arn);
        formatter.field("datastore_status", &self.datastore_status);
        formatter.field("datastore_endpoint", &self.datastore_endpoint);
        formatter.finish()
    }
}
/// See [`DeleteFhirDatastoreOutput`](crate::output::DeleteFhirDatastoreOutput)
pub mod delete_fhir_datastore_output {
    /// A builder for [`DeleteFhirDatastoreOutput`](crate::output::DeleteFhirDatastoreOutput)
    #[non_exhaustive]
    #[derive(std::default::Default, std::clone::Clone, std::cmp::PartialEq, std::fmt::Debug)]
    pub struct Builder {
        pub(crate) datastore_id: std::option::Option<std::string::String>,
        pub(crate) datastore_arn: std::option::Option<std::string::String>,
        pub(crate) datastore_status: std::option::Option<crate::model::DatastoreStatus>,
        pub(crate) datastore_endpoint: std::option::Option<std::string::String>,
    }
    impl Builder {
        /// <p>The AWS-generated ID for the Data Store to be deleted.</p>
        pub fn datastore_id(mut self, input: impl Into<std::string::String>) -> Self {
            self.datastore_id = Some(input.into());
            self
        }
        pub fn set_datastore_id(mut self, input: std::option::Option<std::string::String>) -> Self {
            self.datastore_id = input;
            self
        }
        /// <p>The Amazon Resource Name (ARN) that gives Amazon HealthLake access permission.</p>
        pub fn datastore_arn(mut self, input: impl Into<std::string::String>) -> Self {
            self.datastore_arn = Some(input.into());
            self
        }
        pub fn set_datastore_arn(
            mut self,
            input: std::option::Option<std::string::String>,
        ) -> Self {
            self.datastore_arn = input;
            self
        }
        /// <p>The status of the Data Store that the user has requested to be deleted.
        /// </p>
        pub fn datastore_status(mut self, input: crate::model::DatastoreStatus) -> Self {
            self.datastore_status = Some(input);
            self
        }
        pub fn set_datastore_status(
            mut self,
            input: std::option::Option<crate::model::DatastoreStatus>,
        ) -> Self {
            self.datastore_status = input;
            self
        }
        /// <p>The AWS endpoint for the Data Store the user has requested to be deleted.</p>
        pub fn datastore_endpoint(mut self, input: impl Into<std::string::String>) -> Self {
            self.datastore_endpoint = Some(input.into());
            self
        }
        pub fn set_datastore_endpoint(
            mut self,
            input: std::option::Option<std::string::String>,
        ) -> Self {
            self.datastore_endpoint = input;
            self
        }
        /// Consumes the builder and constructs a [`DeleteFhirDatastoreOutput`](crate::output::DeleteFhirDatastoreOutput)
        pub fn build(self) -> crate::output::DeleteFhirDatastoreOutput {
            crate::output::DeleteFhirDatastoreOutput {
                datastore_id: self.datastore_id,
                datastore_arn: self.datastore_arn,
                datastore_status: self.datastore_status,
                datastore_endpoint: self.datastore_endpoint,
            }
        }
    }
}
impl DeleteFhirDatastoreOutput {
    /// Creates a new builder-style object to manufacture [`DeleteFhirDatastoreOutput`](crate::output::DeleteFhirDatastoreOutput)
    pub fn builder() -> crate::output::delete_fhir_datastore_output::Builder {
        crate::output::delete_fhir_datastore_output::Builder::default()
    }
}

#[non_exhaustive]
#[derive(std::clone::Clone, std::cmp::PartialEq)]
pub struct CreateFhirDatastoreOutput {
    /// <p>The AWS-generated Data Store id. This id is in the output from the initial Data Store
    /// creation call.</p>
    pub datastore_id: std::option::Option<std::string::String>,
    /// <p>The datastore ARN is generated during the creation of the Data Store and can be found in
    /// the output from the initial Data Store creation call.</p>
    pub datastore_arn: std::option::Option<std::string::String>,
    /// <p>The status of the FHIR Data Store. Possible statuses are ‘CREATING’, ‘ACTIVE’, ‘DELETING’,
    /// ‘DELETED’.</p>
    pub datastore_status: std::option::Option<crate::model::DatastoreStatus>,
    /// <p>The AWS endpoint for the created Data Store. For preview, only US-east-1 endpoints are
    /// supported.</p>
    pub datastore_endpoint: std::option::Option<std::string::String>,
}
impl std::fmt::Debug for CreateFhirDatastoreOutput {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut formatter = f.debug_struct("CreateFhirDatastoreOutput");
        formatter.field("datastore_id", &self.datastore_id);
        formatter.field("datastore_arn", &self.datastore_arn);
        formatter.field("datastore_status", &self.datastore_status);
        formatter.field("datastore_endpoint", &self.datastore_endpoint);
        formatter.finish()
    }
}
/// See [`CreateFhirDatastoreOutput`](crate::output::CreateFhirDatastoreOutput)
pub mod create_fhir_datastore_output {
    /// A builder for [`CreateFhirDatastoreOutput`](crate::output::CreateFhirDatastoreOutput)
    #[non_exhaustive]
    #[derive(std::default::Default, std::clone::Clone, std::cmp::PartialEq, std::fmt::Debug)]
    pub struct Builder {
        pub(crate) datastore_id: std::option::Option<std::string::String>,
        pub(crate) datastore_arn: std::option::Option<std::string::String>,
        pub(crate) datastore_status: std::option::Option<crate::model::DatastoreStatus>,
        pub(crate) datastore_endpoint: std::option::Option<std::string::String>,
    }
    impl Builder {
        /// <p>The AWS-generated Data Store id. This id is in the output from the initial Data Store
        /// creation call.</p>
        pub fn datastore_id(mut self, input: impl Into<std::string::String>) -> Self {
            self.datastore_id = Some(input.into());
            self
        }
        pub fn set_datastore_id(mut self, input: std::option::Option<std::string::String>) -> Self {
            self.datastore_id = input;
            self
        }
        /// <p>The datastore ARN is generated during the creation of the Data Store and can be found in
        /// the output from the initial Data Store creation call.</p>
        pub fn datastore_arn(mut self, input: impl Into<std::string::String>) -> Self {
            self.datastore_arn = Some(input.into());
            self
        }
        pub fn set_datastore_arn(
            mut self,
            input: std::option::Option<std::string::String>,
        ) -> Self {
            self.datastore_arn = input;
            self
        }
        /// <p>The status of the FHIR Data Store. Possible statuses are ‘CREATING’, ‘ACTIVE’, ‘DELETING’,
        /// ‘DELETED’.</p>
        pub fn datastore_status(mut self, input: crate::model::DatastoreStatus) -> Self {
            self.datastore_status = Some(input);
            self
        }
        pub fn set_datastore_status(
            mut self,
            input: std::option::Option<crate::model::DatastoreStatus>,
        ) -> Self {
            self.datastore_status = input;
            self
        }
        /// <p>The AWS endpoint for the created Data Store. For preview, only US-east-1 endpoints are
        /// supported.</p>
        pub fn datastore_endpoint(mut self, input: impl Into<std::string::String>) -> Self {
            self.datastore_endpoint = Some(input.into());
            self
        }
        pub fn set_datastore_endpoint(
            mut self,
            input: std::option::Option<std::string::String>,
        ) -> Self {
            self.datastore_endpoint = input;
            self
        }
        /// Consumes the builder and constructs a [`CreateFhirDatastoreOutput`](crate::output::CreateFhirDatastoreOutput)
        pub fn build(self) -> crate::output::CreateFhirDatastoreOutput {
            crate::output::CreateFhirDatastoreOutput {
                datastore_id: self.datastore_id,
                datastore_arn: self.datastore_arn,
                datastore_status: self.datastore_status,
                datastore_endpoint: self.datastore_endpoint,
            }
        }
    }
}
impl CreateFhirDatastoreOutput {
    /// Creates a new builder-style object to manufacture [`CreateFhirDatastoreOutput`](crate::output::CreateFhirDatastoreOutput)
    pub fn builder() -> crate::output::create_fhir_datastore_output::Builder {
        crate::output::create_fhir_datastore_output::Builder::default()
    }
}