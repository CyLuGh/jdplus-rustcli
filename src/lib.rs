mod jdplus_main_ws_v1 {
    tonic::include_proto!("jdplus.main.ws.v1");
}

pub mod communication_manager;
pub mod models;

pub fn add(left: u64, right: u64) -> u64 {
    left + right
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let result = add(2, 2);
        assert_eq!(result, 4);
    }
}
