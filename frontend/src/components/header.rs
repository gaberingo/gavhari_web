use yew::prelude::*;

#[function_component(Header)]
pub fn header() -> Html {
    html! {
        <div class="navbar bg-neutral text-neutral-content">
            <div class="navbar-start">
                <div class="join">
                    <label for="my-drawer" class="btn btn-square btn-ghost join-item">
                        <svg xmlns="http://www.w3.org/2000/svg" fill="none" viewBox="0 0 24 24" class="inline-block h-5 w-5 stroke-current"> <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M4 6h16M4 12h16M4 18h16"></path> </svg>
                    </label>
                    <div class="dropdown dropdown-right join-item">
                        <div tabindex="0" role="button" class="btn btn-ghost text-xl">{"Purchase Order"}</div>
                        <ul tabindex="0" class="dropdown-content menu xl:menu-horizontal bg-base-200 rounded-box w-56">
                            <li>
                                <h2 class="menu-title">{"Purchase"}</h2>
                                <ul>
                                <li><a>{"Item 1"}</a></li>
                                <li><a>{"Item 2"}</a></li>
                                <li><a>{"Item 3"}</a></li>
                                </ul>
                            </li>
                            <li>
                                <h2 class="menu-title">{"Sales"}</h2>
                                <ul>
                                <li><a>{"Item 1"}</a></li>
                                <li><a>{"Item 2"}</a></li>
                                <li><a>{"Item 3"}</a></li>
                                </ul>
                            </li>
                            <li>
                                <h2 class="menu-title">{"Inventory"}</h2>
                                <ul>
                                <li><a>{"Item 1"}</a></li>
                                <li><a>{"Item 2"}</a></li>
                                <li><a>{"Item 3"}</a></li>
                                </ul>
                            </li>
                        </ul>
                    </div>
                </div>
            </div>
            <div class="navbar-center">
                <div class="breadcrumbs max-w-xs text-sm">
                // TODO: Replace with dynamic breadcrumbs
                    <ul>
                        <li>{"Long text 1"}</li>
                        <li>{"Long text 2"}</li>
                        <li>{"Long text 3"}</li>
                    </ul>
                </div>
            </div>
            <div class="navbar-end gap-x-2">
                <div class="join">
                    <input type="text" placeholder="Search" class="input join-item w-40 md:w-auto" />
                    <button class="btn btn-ghost join-item">
                        <svg xmlns="http://www.w3.org/2000/svg" class="h-5 w-5" fill="none" viewBox="0 0 24 24" stroke="currentColor">
                            <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M21 21l-6-6m2-5a7 7 0 11-14 0 7 7 0 0114 0z" />
                        </svg>
                    </button>
                </div>

                <div class="dropdown dropdown-end">
                    <div tabindex="0" role="button" class="flex gap-x-4 items-center w-full">
                        <div class="btn btn-ghost btn-circle avatar">
                            <div class="w-10 rounded-full">
                                <img
                                    alt="Tailwind CSS Navbar component"
                                    src="https://img.daisyui.com/images/stock/photo-1534528741775-53994a69daeb.webp" />
                            </div>
                        </div>
                    </div>
                    <ul
                        tabindex="0"
                        class="menu menu-sm dropdown-content bg-neutral rounded-box z-1 mt-3 w-52 p-2">
                        <li>
                        <a class="justify-between">
                                {"Profile"}
                            <span class="badge">{"New"}</span>
                        </a>
                        </li>
                        <li><a>{"Settings"}</a></li>
                        <li><a>{"Logout"}</a></li>
                    </ul>
                </div>

            </div>
        </div>
    }
}
