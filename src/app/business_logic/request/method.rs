use crate::app::app::App;
use crate::models::method::Method;

impl App<'_> {
    pub fn modify_request_method(&mut self, method: Method, collection_index: usize, request_index: usize) {
        let local_selected_request = self.get_request_as_local_from_indexes(&(collection_index, request_index));

        {
            let mut selected_request = local_selected_request.write();

            selected_request.method = method;
        }

        self.save_collection_to_file(collection_index);
    }
}