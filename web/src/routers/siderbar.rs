use yew_icons::IconId;
use yew_router::prelude::*;


#[derive(Clone, PartialEq, Routable)]
pub enum AppRoutes {
    #[at("/app/welcome")]
    Welcome,
    #[at("/app/:path")]
    Other { path: String },
}



pub struct Router {
    pub path: &'static str,
    pub name: &'static str,
    pub icon_id: IconId,
    pub sub_routers: Option<&'static [Router]>,
}

pub const SIDERBAR_ROUTERS: &[Router] = &[
    Router {
        path:"dashboard",
        name:"仪表盘",
        icon_id: IconId::HeroiconsOutlineSquares2X2,
        sub_routers: None,
    },
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
