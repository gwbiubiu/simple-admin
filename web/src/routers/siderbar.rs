use yew_icons::IconId;
pub struct Router {
    pub path: &'static str,
    pub name: &'static str,
    pub icon_id: IconId,
    pub sub_routers: Option<&'static [Router]>,
}

pub const SIDERBAR_ROUTERS: &[Router] = &[
    Router {
        path:"roles",
        name:"角色管理",
        icon_id: IconId::LucideUserCog
        ,
        sub_routers: None,
    },
    Router {
        path:"users",
        name:"用户管理",
        icon_id: IconId::LucideUsers        ,
        sub_routers: None,
    },
    Router {
        path:"auths",
        name:"API管理",
        icon_id: IconId::BootstrapRouter,
        sub_routers: None,
    },
];
