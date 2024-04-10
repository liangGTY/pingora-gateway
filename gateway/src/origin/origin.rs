struct Origin {}

struct OriginName {}

struct BaseOriginManager {}

trait OriginManager {
    fn get_or_create_origin(&self, origin_name: OriginName) -> Origin;
}

impl OriginManager for BaseOriginManager {
    fn get_or_create_origin(&self, origin_name: OriginName) -> Origin {
        todo!()
    }
}