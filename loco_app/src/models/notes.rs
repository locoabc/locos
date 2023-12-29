use sea_orm::entity::prelude::*;

use super::_entities::notes::ActiveModel;

impl ActiveModelBehavior for ActiveModel {
    // extend activemodel below (keep comment for generators)
    async fn before_save<C>(self, _db: &C, insert: bool) -> Result<Self, DbErr>
    where
        C: ConnectionTrait,
        {
            println!("This is happening before we save something!");
            Ok(self)
    }
}
