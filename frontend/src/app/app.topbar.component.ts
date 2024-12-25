import {Component} from '@angular/core';
import {AppMainComponent} from './app.main.component';

@Component({
    selector: 'app-topbar',
    template: `
        <div class="layout-topbar">
			<div class="layout-topbar-wrapper">
                <div class="layout-topbar-left">
					<div class="layout-topbar-logo-wrapper">
						<a href="#" class="layout-topbar-logo">
							<img src="assets/layout/images/logo-mirage@2x.png" alt="mirage-layout" />
							<span class="app-name">Mirage</span>
						</a>
					</div>

					<a href="#" class="sidebar-menu-button" (click)="appMain.onMenuButtonClick($event)">
						<i class="pi pi-bars"></i>
					</a>

					<a href="#" class="megamenu-mobile-button" (click)="appMain.onMegaMenuMobileButtonClick($event)">
						<i class="pi pi-align-right megamenu-icon"></i>
					</a>

					<a href="#" class="topbar-menu-mobile-button" (click)="appMain.onTopbarMobileMenuButtonClick($event)">
						<i class="pi pi-ellipsis-v"></i>
					</a>

					<div class="layout-megamenu-wrapper">
						<a class="layout-megamenu-button" href="#" (click)="appMain.onMegaMenuButtonClick($event)">
							<i class="pi pi-comment"></i>
							Mega Menu
						</a>
						<ul class="layout-megamenu" [ngClass]="{'layout-megamenu-active fadeInDown': appMain.megaMenuActive}"
                            (click)="appMain.onMegaMenuClick($event)">
							<li [ngClass]="{'active-topmenuitem': activeItem === 1}" (click)="mobileMegaMenuItemClick(1)">
								<a href="#">JavaServer Faces <i class="pi pi-angle-down"></i></a>
								<ul>
									<li class="active-row ">
										<i class="pi pi-circle-on"></i>
										<span>
                                        <h5>PrimeFaces</h5>
                                        <span>UI Components for JSF</span>
                                    </span>
									</li>
									<li>
										<i class="pi pi-circle-on"></i>
										<span>
                                        <h5>Premium Templates</h5>
                                        <span>UI Components for JSF</span>
                                    </span>
									</li>
									<li>
										<i class="pi pi-circle-on"></i>
										<span>
                                        <h5>Extensions</h5>
                                        <span>UI Components for JSF</span>
                                    </span>
									</li>
								</ul>
							</li>
							<li [ngClass]="{'active-topmenuitem': activeItem === 2}" (click)="mobileMegaMenuItemClick(2)">
								<a href="#">Angular <i class="pi pi-angle-down"></i></a>
								<ul>
									<li>
										<i class="pi pi-circle-on"></i>
										<span>
                                        <h5>PrimeNG</h5>
                                        <span>UI Components for Angular</span>
                                    </span>

									</li>
									<li>
										<i class="pi pi-circle-on"></i>
										<span>
                                        <h5>Premium Templates</h5>
                                        <span>UI Components for Angular</span>
                                    </span>
									</li>
								</ul>
							</li>
							<li [ngClass]="{'active-topmenuitem': activeItem === 3}" (click)="mobileMegaMenuItemClick(3)">
								<a href="#">React <i class="pi pi-angle-down"></i></a>
								<ul>
									<li>
										<i class="pi pi-circle-on"></i>
										<span>
                                        <h5>PrimeReact</h5>
                                        <span>UI Components for React</span>
                                    </span>
									</li>
									<li class="active-row">
										<i class="pi pi-circle-on"></i>
										<span>
                                        <h5>Premium Templates</h5>
                                        <span>UI Components for React</span>
                                    </span>
									</li>
								</ul>
							</li>
						</ul>
					</div>
                </div>
                <div class="layout-topbar-right fadeInDown">
					<ul class="layout-topbar-actions">
						<li #search class="search-item topbar-item" [ngClass]="{'active-topmenuitem': appMain.activeTopbarItem === search}">
							<a href="#" class="topbar-search-mobile-button" (click)="appMain.onTopbarItemClick($event,search)">
								<i class="topbar-icon pi pi-search"></i>
							</a>
							<ul class="search-item-submenu fadeInDown" (click)="appMain.topbarItemClick = true">
								<li>
                                    <span class="md-inputfield search-input-wrapper">
                                        <input pInputText placeholder="Search..."/>
                                        <i class="pi pi-search"></i>
                                    </span>
                                </li>
                            </ul>
                        </li>
						<li #calendar class="topbar-item" [ngClass]="{'active-topmenuitem': appMain.activeTopbarItem === calendar}">
							<a href="#" (click)="appMain.onTopbarItemClick($event,calendar)">
								<i class="topbar-icon pi pi-calendar"></i>
							</a>
							<ul class="fadeInDown" (click)="appMain.topbarItemClick = true">
								<li class="layout-submenu-header">
									<h1>Calendar</h1>
								</li>
								<li class="calendar">
                                    <p-calendar [inline]="true"></p-calendar>
								</li>
							</ul>
						</li>

						<li #message class="topbar-item" [ngClass]="{'active-topmenuitem': appMain.activeTopbarItem === message}">
							<a href="#" (click)="appMain.onTopbarItemClick($event,message)">
								<i class="topbar-icon pi pi-inbox"></i>
							</a>
							<ul class="fadeInDown">
								<li class="layout-submenu-header">
									<h1>Messages</h1>
									<span>Today, you have new 4 messages</span>
								</li>
								<li class="layout-submenu-item">
									<img src="assets/layout/images/topbar/avatar-cayla.png" alt="mirage-layout" width="35" />
									<div class="menu-text">
										<p>Override the digital divide</p>
										<span>Cayla Brister</span>
									</div>
									<i class="pi pi-angle-right"></i>
								</li>
								<li class="layout-submenu-item">
									<img src="assets/layout/images/topbar/avatar-gabie.png" alt="mirage-layout" width="35" />
									<div class="menu-text">
										<p>Nanotechnology immersion</p>
										<span>Gabie Sheber</span>
									</div>
									<i class="pi pi-angle-right"></i>
								</li>
								<li class="layout-submenu-item">
									<img src="assets/layout/images/topbar/avatar-gaspar.png" alt="mirage-layout" width="35" />
									<div class="menu-text">
										<p>User generated content</p>
										<span>Gaspar Antunes</span>
									</div>
									<i class="pi pi-angle-right"></i>
								</li>
								<li class="layout-submenu-item">
									<img src="assets/layout/images/topbar/avatar-tatiana.png" alt="mirage-layout" width="35" />
									<div class="menu-text">
										<p>The holistic world view</p>
										<span>Tatiana Gagelman</span>
									</div>
									<i class="pi pi-angle-right"></i>
								</li>
							</ul>
						</li>

						<li #gift class="topbar-item" [ngClass]="{'active-topmenuitem': appMain.activeTopbarItem === gift}">
							<a href="#" (click)="appMain.onTopbarItemClick($event,gift)">
								<i class="topbar-icon pi pi-envelope"></i>
							</a>
							<ul class="fadeInDown">
								<li class="layout-submenu-header">
									<h1>Deals</h1>
								</li>

								<li class="deals">
									<ul>
										<li>
											<img src="assets/layout/images/topbar/deal-icon-sapphire.png" alt="mirage-layout" width="35" />
											<div class="menu-text">
												<p>Sapphire</p>
												<span>Angular</span>
											</div>
											<i class="pi pi-angle-right"></i>
										</li>
										<li>
											<img src="assets/layout/images/topbar/deal-icon-roma.png" alt="mirage-layout" width="35" />
											<div class="menu-text">
												<p>Roma</p>
												<span>Minimalism</span>
											</div>
											<i class="pi pi-angle-right"></i>
										</li>
										<li>
											<img src="assets/layout/images/topbar/deal-icon-babylon.png" alt="mirage-layout" width="35" />
											<div class="menu-text">
												<p>Babylon</p>
												<span>Powerful</span>
											</div>
											<i class="pi pi-angle-right"></i>
										</li>
									</ul>
									<ul>
										<li>
											<img src="assets/layout/images/topbar/deal-icon-harmony.png" alt="mirage-layout" width="35" />
											<div class="menu-text">
												<p>Harmony</p>
												<span>USWDS</span>
											</div>
											<i class="pi pi-angle-right"></i>
										</li>
										<li>
											<img src="assets/layout/images/topbar/deal-icon-prestige.png" alt="mirage-layout" width="35" />
											<div class="menu-text">
												<p>Prestige</p>
												<span>Elegancy</span>
											</div>
											<i class="pi pi-angle-right"></i>
										</li>
										<li>
											<img src="assets/layout/images/topbar/deal-icon-ultima.png" alt="mirage-layout" width="35" />
											<div class="menu-text">
												<p>Ultima</p>
												<span>Material</span>
											</div>
											<i class="pi pi-angle-right"></i>
										</li>
									</ul>
								</li>
							</ul>
						</li>

						<li #profile class="topbar-item profile-item" [ngClass]="{'active-topmenuitem': appMain.activeTopbarItem === profile}">
							<a href="#" (click)="appMain.onTopbarItemClick($event,profile)">
                            <span class="profile-image-wrapper">
                                <img src="assets/layout/images/topbar/avatar-eklund.png" alt="mirage-layout" />
                            </span>
								<span class="profile-info-wrapper">
                                <h3>Olivia Eklund</h3>
                                <span>Design</span>
                            </span>
							</a>
							<ul class="profile-item-submenu fadeInDown">
								<li class="profile-submenu-header">
									<div class="performance">
										<span>Weekly Performance</span>
										<img src="assets/layout/images/topbar/asset-bars.svg" alt="mirage-layout" />
									</div>
									<div class="profile">
										<img src="assets/layout/images/topbar/avatar-eklund.png" alt="mirage-layout"
														width="40" />
										<h1>Olivia Eklund</h1>
										<span>Design</span>
									</div>
								</li>
								<li class="layout-submenu-item">
									<i class="pi pi-list icon icon-1"></i>
									<div class="menu-text">
										<p>Tasks</p>
										<span>3 open issues</span>
									</div>
									<i class="pi pi-angle-right"></i>
								</li>
								<li class="layout-submenu-item">
									<i class="pi pi-shopping-cart icon icon-2"></i>
									<div class="menu-text">
										<p>Payments</p>
										<span>24 new</span>
									</div>
									<i class="pi pi-angle-right"></i>
								</li>
								<li class="layout-submenu-item">
									<i class="pi pi-users icon icon-3"></i>
									<div class="menu-text">
										<p>Clients</p>
										<span>+80%</span>
									</div>
									<i class="pi pi-angle-right"></i>
								</li>
								<li class="layout-submenu-footer">
									<button class="signout-button">Sign Out</button>
									<button class="buy-mirage-button">Buy Mirage</button>
								</li>
							</ul>
						</li>
						<li>
							<a href="#" class="layout-rightpanel-button" (click)="appMain.onRightPanelButtonClick($event)">
								<i class="pi pi-arrow-left"></i>
							</a>
						</li>
                    </ul>

					<ul class="profile-mobile-wrapper">
						<li #mobileProfile class="topbar-item profile-item" [ngClass]="{'active-topmenuitem': appMain.activeTopbarItem === mobileProfile}">
							<a href="#" (click)="appMain.onTopbarItemClick($event,mobileProfile)">
                            <span class="profile-image-wrapper">
                                <img src="assets/layout/images/topbar/avatar-eklund.png" alt="mirage-layout" />
                            </span>
								<span class="profile-info-wrapper">
                                <h3>Olivia Eklund</h3>
                                <span>Design</span>
                            </span>
							</a>
							<ul class="fadeInDown">
								<li class="profile-submenu-header">
									<div class="performance">
										<span>Weekly Performance</span>
										<img src="assets/layout/images/topbar/asset-bars.svg" alt="mirage-layout" />
									</div>
									<div class="profile">
										<img src="assets/layout/images/topbar/avatar-eklund.png" alt="mirage-layout" width="45" />
										<h1>Olivia Eklund</h1>
										<span>Design</span>
									</div>
								</li>
								<li>
									<i class="pi pi-list icon icon-1"></i>
									<div class="menu-text">
										<p>Tasks</p>
										<span>3 open issues</span>
									</div>
									<i class="pi pi-angle-right"></i>
								</li>
								<li>
									<i class="pi pi-shopping-cart icon icon-2"></i>
									<div class="menu-text">
										<p>Payments</p>
										<span>24 new</span>
									</div>
									<i class="pi pi-angle-right"></i>
								</li>
								<li>
									<i class="pi pi-users icon icon-3"></i>
									<div class="menu-text">
										<p>Clients</p>
										<span>+80%</span>
									</div>
									<i class="pi pi-angle-right"></i>
								</li>
								<li class="layout-submenu-footer">
									<button class="signout-button">Sign Out</button>
									<button class="buy-mirage-button">Buy Mirage</button>
								</li>
							</ul>
						</li>
					</ul>
                </div>
            </div>
        </div>
    `
})
export class AppTopBarComponent {

    activeItem: number;

    constructor(public appMain: AppMainComponent) {}

    mobileMegaMenuItemClick(index) {
        this.appMain.megaMenuMobileClick = true;
        this.activeItem = this.activeItem === index ? null : index;
    }

}
