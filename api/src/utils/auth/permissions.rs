use bitflags::bitflags;

bitflags! {
    pub struct Permissions: u32 {
        const SUPER_ADMIN = 1 << 4;
        const USER_READ = 1;
        const USER_CREATE = 1 << 1;
        const USER_DELETE = 1 << 2;
        const USER_UPDATE = 1 << 3;
        const PERMISSIONS_READ = 1 << 4;
        const PERMISSIONS_UPDATE = 1 << 5;
        /// Freezer permissions include the drawer permissions since they are inherently part
        /// of the freezer itself.
        const FREEZER_CREATE = 1 << 6;
        const FREEZER_DELETE = 1 << 7;
        const FREEZER_UPDATE = 1 << 8;
        const PRODUCT_CREATE = 1 << 9;
        const PRODUDUCT_DELETE = 1 << 10;
        const PRODUCT_UPDATE = 1 << 11;
        const STORAGE_CREATE = 1 << 12;
        const STORAGE_DELETE = 1 << 13;
        const STORAGE_UPDATE = 1 << 14;
    }
}

pub enum Roles {
    Maintainer,
    Admin,
    LocalAdmin,
    User,
}
impl Roles {
    pub fn permissions(&self) -> Permissions {
        match self {
            Roles::Maintainer => Permissions::all(),
            Roles::Admin => Permissions::all(),
            Roles::LocalAdmin => Permissions::USER_READ | Permissions::USER_UPDATE,
            Roles::User => Permissions::USER_READ,
        }
    }
    pub fn has_permission(&self, permissions: Permissions) -> bool {
        self.permissions().contains(permissions)
    }
}
