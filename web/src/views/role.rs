use yew::prelude::*;
struct Role {
    id: u32,
    name: String,
    description: String,
}


pub struct RoleList;


impl Component for RoleList {
    type Message = ();
    type Properties = ();

    fn create(_ctx: &Context<Self>) -> Self {
        Self {}
    }

    fn view(&self, _ctx: &Context<Self>) -> Html {
        let roles = vec![
            Role { id: 1, name: "管理员".to_string(), description: "系统管理员".to_string() },
            Role { id: 2, name: "编辑".to_string(), description: "内容编辑".to_string() },
            Role { id: 3, name: "访客".to_string(), description: "只读权限".to_string() },
        ];
        html! {
        <div>
            <h2>{"角色管理"}</h2>
            <table class="table table-striped">
                <thead>
                    <tr>
                        <th>{"ID"}</th>
                        <th>{"角色名"}</th>
                        <th>{"描述"}</th>
                        <th>{"操作"}</th>
                    </tr>
                </thead>
                <tbody>
                    {roles.iter().map(|role| html! {
                        <tr key={role.id}>
                            <td>{role.id}</td>
                            <td>{&role.name}</td>
                            <td>{&role.description}</td>
                            <td>
                                <button class="btn btn-sm btn-info me-2">{"编辑"}</button>
                                <button class="btn btn-sm btn-danger">{"删除"}</button>
                            </td>
                        </tr>
                    }).collect::<Html>()}
                </tbody>
            </table>
            <nav aria-label="Page navigation example">
              <ul class="pagination">
                <li class="page-item">
                  <a class="page-link" href="#" aria-label="Previous">
                    <span aria-hidden="true">{"&laquo;"}</span>
                  </a>
                </li>
                <li class="page-item"><a class="page-link" href="#">{"1"}</a></li>
                <li class="page-item"><a class="page-link" href="#">{"2"}</a></li>
                <li class="page-item"><a class="page-link" href="#">{"3"}</a></li>
                <li class="page-item">
                  <a class="page-link" href="#" aria-label="Next">
                    <span aria-hidden="true">{"&raquo;"}</span>
                  </a>
                </li>
              </ul>
            </nav>
        </div>
    }
    }
}