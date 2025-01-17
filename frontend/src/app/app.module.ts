import { NgModule, CUSTOM_ELEMENTS_SCHEMA } from "@angular/core";
import { FormsModule } from "@angular/forms";
import { HTTP_INTERCEPTORS, HttpClientModule } from "@angular/common/http";
import { BrowserModule } from "@angular/platform-browser";
import { BrowserAnimationsModule } from "@angular/platform-browser/animations";
import { LocationStrategy, PathLocationStrategy } from "@angular/common";

// PrimeNG Components for demos
import { AccordionModule } from "primeng/accordion";
import { AutoCompleteModule } from "primeng/autocomplete";
import { AvatarModule } from "primeng/avatar";
import { AvatarGroupModule } from "primeng/avatargroup";
import { BadgeModule } from "primeng/badge";
import { BreadcrumbModule } from "primeng/breadcrumb";
import { ButtonModule } from "primeng/button";
import { CalendarModule } from "primeng/calendar";
import { CardModule } from "primeng/card";
import { CarouselModule } from "primeng/carousel";
import { CascadeSelectModule } from "primeng/cascadeselect";
import { ChartModule } from "primeng/chart";
import { CheckboxModule } from "primeng/checkbox";
import { ChipModule } from "primeng/chip";
import { ChipsModule } from "primeng/chips";
import { ConfirmDialogModule } from "primeng/confirmdialog";
import { ConfirmPopupModule } from "primeng/confirmpopup";
import { ColorPickerModule } from "primeng/colorpicker";
import { ContextMenuModule } from "primeng/contextmenu";
import { DataViewModule } from "primeng/dataview";
import { DialogModule } from "primeng/dialog";
import { DividerModule } from "primeng/divider";
import { DropdownModule } from "primeng/dropdown";
import { FieldsetModule } from "primeng/fieldset";
import { FileUploadModule } from "primeng/fileupload";
import { GalleriaModule } from "primeng/galleria";
import { ImageModule } from "primeng/image";
import { InplaceModule } from "primeng/inplace";
import { InputNumberModule } from "primeng/inputnumber";
import { InputMaskModule } from "primeng/inputmask";
import { InputSwitchModule } from "primeng/inputswitch";
import { InputTextModule } from "primeng/inputtext";
import { InputTextareaModule } from "primeng/inputtextarea";
import { KnobModule } from "primeng/knob";
import { ListboxModule } from "primeng/listbox";
import { MegaMenuModule } from "primeng/megamenu";
import { MenuModule } from "primeng/menu";
import { MenubarModule } from "primeng/menubar";
import { MessagesModule } from "primeng/messages";
import { MessageModule } from "primeng/message";
import { MultiSelectModule } from "primeng/multiselect";
import { OrderListModule } from "primeng/orderlist";
import { OrganizationChartModule } from "primeng/organizationchart";
import { OverlayPanelModule } from "primeng/overlaypanel";
import { PaginatorModule } from "primeng/paginator";
import { PanelModule } from "primeng/panel";
import { PanelMenuModule } from "primeng/panelmenu";
import { PasswordModule } from "primeng/password";
import { PickListModule } from "primeng/picklist";
import { ProgressBarModule } from "primeng/progressbar";
import { RadioButtonModule } from "primeng/radiobutton";
import { RatingModule } from "primeng/rating";
import { RippleModule } from "primeng/ripple";
import { ScrollPanelModule } from "primeng/scrollpanel";
import { ScrollTopModule } from "primeng/scrolltop";
import { SelectButtonModule } from "primeng/selectbutton";
import { SidebarModule } from "primeng/sidebar";
import { SkeletonModule } from "primeng/skeleton";
import { SlideMenuModule } from "primeng/slidemenu";
import { SliderModule } from "primeng/slider";
import { SplitButtonModule } from "primeng/splitbutton";
import { SplitterModule } from "primeng/splitter";
import { StepsModule } from "primeng/steps";
import { TabMenuModule } from "primeng/tabmenu";
import { TableModule } from "primeng/table";
import { TabViewModule } from "primeng/tabview";
import { TagModule } from "primeng/tag";
import { TerminalModule } from "primeng/terminal";
import { TieredMenuModule } from "primeng/tieredmenu";
import { TimelineModule } from "primeng/timeline";
import { ToastModule } from "primeng/toast";
import { ToggleButtonModule } from "primeng/togglebutton";
import { ToolbarModule } from "primeng/toolbar";
import { TooltipModule } from "primeng/tooltip";
import { TreeModule } from "primeng/tree";
import { TreeTableModule } from "primeng/treetable";
import { VirtualScrollerModule } from "primeng/virtualscroller";
import { FullCalendarModule } from "@fullcalendar/angular";

// Application Components
import { AppComponent } from "./app.component";
import { AppRoutingModule } from "./app-routing.module";
import { AppMainComponent } from "./app.main.component";
// import { AppNotfoundComponent } from './pages/app.notfound.component';
// import { AppErrorComponent } from './pages/app.error.component';
// import { AppAccessdeniedComponent } from './pages/app.accessdenied.component';
import { AppLoginComponent } from "./pages/app.login.component";
// import { AppCrudComponent } from './pages/app.crud.component';
// import { AppCalendarComponent } from './pages/app.calendar.component';
// import { AppTimelineDemoComponent } from './pages/app.timelinedemo.component';
import { AppMenuComponent } from "./app.menu.component";
import { AppMenuitemComponent } from "./app.menuitem.component";
import { AppBreadcrumbComponent } from "./app.breadcrumb.component";
import { AppConfigComponent } from "./app.config.component";
import { AppRightPanelComponent } from "./app.rightpanel.component";
import { AppTopBarComponent } from "./app.topbar.component";
import { AppFooterComponent } from "./app.footer.component";

// Demo pages
// import {DashboardDemoComponent} from './demo/view/dashboarddemo.component';
// import {FormLayoutDemoComponent} from './demo/view/formlayoutdemo.component';
// import {FloatLabelDemoComponent} from './demo/view/floatlabeldemo.component';
// import {InvalidStateDemoComponent} from './demo/view/invalidstatedemo.component';
// import {InputDemoComponent} from './demo/view/inputdemo.component';
// import {ButtonDemoComponent} from './demo/view/buttondemo.component';
// import {TableDemoComponent} from './demo/view/tabledemo.component';
// import {ListDemoComponent} from './demo/view/listdemo.component';
// import {TreeDemoComponent} from './demo/view/treedemo.component';
// import {PanelsDemoComponent} from './demo/view/panelsdemo.component';
// import {OverlaysDemoComponent} from './demo/view/overlaysdemo.component';
// import {MediaDemoComponent} from './demo/view/mediademo.component';
// import {MenusComponent} from './demo/view/menus/menus.component';
// import {MessagesDemoComponent} from './demo/view/messagesdemo.component';
// import {MiscDemoComponent} from './demo/view/miscdemo.component';
// import {EmptyDemoComponent} from './demo/view/emptydemo.component';
// import {ChartsDemoComponent} from './demo/view/chartsdemo.component';
// import {FileDemoComponent} from './demo/view/filedemo.component';
// import {DocumentationComponent} from './demo/view/documentation.component';
// import { IconsComponent } from './utilities/icons.component';
import { BlocksComponent } from "./blocks/blocks/blocks.component";
import { BlockViewer } from "./blocks/blockviewer/blockviewer.component";

// Demo services
// import {CountryService} from './demo/service/countryservice';
// import {EventService} from './demo/service/eventservice';
// import {NodeService} from './demo/service/nodeservice';
// import {CustomerService} from './demo/service/customerservice';
// import {PhotoService} from './demo/service/photoservice';
// import {ProductService} from './demo/service/productservice';
// import {IconService} from './demo/service/iconservice';
// import {ConfigService} from './demo/service/app.config.service';
import { ConfigService } from "./service/app.config.service";

// Application services
import { BreadcrumbService } from "./breadcrumb.service";
import { MenuService } from "./app.menu.service";
import { AppCodeModule } from "./blocks/app-code/app.code.component";
import { AuthService } from "./service/auth.service";
import { AccountComponent } from "./pages/account/account.component";
import { JwtInterceptor } from "./service/jwtinterceptor.service";
import { MemberComponent } from "./pages/member/member.component";
import { MemberListComponent } from './pages/member-list/member-list.component';

@NgModule({
  imports: [
    BrowserModule,
    FormsModule,
    AppRoutingModule,
    AppCodeModule,
    HttpClientModule,
    BrowserAnimationsModule,
    AccordionModule,
    AutoCompleteModule,
    AvatarModule,
    AvatarGroupModule,
    BadgeModule,
    BreadcrumbModule,
    ButtonModule,
    CalendarModule,
    CardModule,
    CarouselModule,
    CascadeSelectModule,
    ChartModule,
    CheckboxModule,
    ChipModule,
    ChipsModule,
    ConfirmDialogModule,
    ConfirmPopupModule,
    ColorPickerModule,
    ContextMenuModule,
    DataViewModule,
    DialogModule,
    DividerModule,
    DropdownModule,
    FieldsetModule,
    FileUploadModule,
    GalleriaModule,
    FullCalendarModule,
    ImageModule,
    InplaceModule,
    InputNumberModule,
    InputMaskModule,
    InputSwitchModule,
    InputTextModule,
    InputTextareaModule,
    KnobModule,
    ListboxModule,
    MegaMenuModule,
    MenuModule,
    MenubarModule,
    MessageModule,
    MessagesModule,
    MultiSelectModule,
    OrderListModule,
    OrganizationChartModule,
    OverlayPanelModule,
    PaginatorModule,
    PanelModule,
    PanelMenuModule,
    PasswordModule,
    PickListModule,
    ProgressBarModule,
    RadioButtonModule,
    RatingModule,
    RippleModule,
    ScrollPanelModule,
    ScrollTopModule,
    SelectButtonModule,
    SidebarModule,
    SkeletonModule,
    SlideMenuModule,
    SliderModule,
    SplitButtonModule,
    SplitterModule,
    StepsModule,
    TableModule,
    TabMenuModule,
    TabViewModule,
    TagModule,
    TerminalModule,
    TimelineModule,
    TieredMenuModule,
    ToastModule,
    ToggleButtonModule,
    ToolbarModule,
    TooltipModule,
    TreeModule,
    TreeTableModule,
    VirtualScrollerModule,
  ],
  declarations: [
    AppComponent,
    AppMainComponent,
    AppMenuComponent,
    AppMenuitemComponent,
    AppTopBarComponent,
    AppFooterComponent,
    AppRightPanelComponent,
    AppConfigComponent,
    AppBreadcrumbComponent,
    // AppNotfoundComponent,
    // AppErrorComponent,
    // AppAccessdeniedComponent,
    AppLoginComponent,
    // AppCrudComponent,
    // AppCalendarComponent,
    // AppTimelineDemoComponent,
    // DashboardDemoComponent,
    // FormLayoutDemoComponent,
    // FloatLabelDemoComponent,
    // InvalidStateDemoComponent,
    // InputDemoComponent,
    // ButtonDemoComponent,
    // TableDemoComponent,
    // ListDemoComponent,
    // TreeDemoComponent,
    // PanelsDemoComponent,
    // OverlaysDemoComponent,
    // MediaDemoComponent,
    // MenusComponent,
    // MessagesDemoComponent,
    // MessagesDemoComponent,
    // MiscDemoComponent,
    // ChartsDemoComponent,
    // EmptyDemoComponent,
    // FileDemoComponent,
    // DocumentationComponent,
    // IconsComponent,
    BlocksComponent,
    BlockViewer,
    AccountComponent,
    MemberComponent,
    MemberListComponent,
  ],
  providers: [
    { provide: LocationStrategy, useClass: PathLocationStrategy },
    { provide: HTTP_INTERCEPTORS, useClass: JwtInterceptor, multi: true },
    // CountryService, CustomerService, EventService, IconService, NodeService,
    // PhotoService, ProductService,
    ConfigService,
    MenuService,
    BreadcrumbService,
    AuthService,
  ],
  bootstrap: [AppComponent],
})
export class AppModule {}
