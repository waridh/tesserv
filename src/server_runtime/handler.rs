/*! Module that provides handlers for the different endpoints
 */

use warp::{Rejection, Reply, multipart::FormData};

pub async fn run_test() {}

pub async fn handle_submit(form: FormData) -> Result<impl Reply, Rejection> {
    todo!();
}
