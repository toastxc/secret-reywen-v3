#[macro_export]
macro_rules! derive_a1 {
    ($(($method:ident, $method_path:path, $data_type:ty, $return_type:ty)),* $(,)?) => {
        $(
            pub async fn $method(
                &self,
                data1: $data_type,
            ) -> Result<$return_type, DeltaError> {
                $method_path(&self.connection, data1).await
            }
        )*
    };
}

#[macro_export]
macro_rules! derive_a2 {
    ($(($method:ident, $method_path:path, $data_type:ty,  $data_type2:ty, $return_type:ty)),* $(,)?) => {
        $(
            pub async fn $method(
                &self,
                data1: $data_type,
              data2: $data_type2,
            ) -> Result<$return_type, DeltaError> {
                $method_path(&self.connection, data1, data2).await
            }
        )*
    };
}
#[macro_export]
macro_rules! derive_a3 {
    ($(($method:ident, $method_path:path, $data_type:ty,  $data_type2:ty,  $data_type3:ty, $return_type:ty)),* $(,)?) => {
        $(
            pub async fn $method(
                &self,
                data1: $data_type,
              data2: $data_type2,
              data3: $data_type3,
            ) -> Result<$return_type, DeltaError> {
                $method_path(&self.connection, data1, data2, data3).await
            }
        )*
    };
}
#[macro_export]
macro_rules! derive_a4 {
    ($(($method:ident, $method_path:path, $data_type:ty,  $data_type2:ty,  $data_type3:ty,  $data_type4:ty, $return_type:ty)),* $(,)?) => {
        $(
            pub async fn $method(
                &self,
                data1: $data_type,
              data2: $data_type2,
              data3: $data_type3,
             data4: $data_type4,
            ) -> Result<$return_type, DeltaError> {
                $method_path(&self.connection, data1, data2, data3, data4).await
            }
        )*
    };
}
