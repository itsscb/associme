/* tslint:disable:no-unused-variable */

import { async, TestBed } from '@angular/core/testing';
import { RouterTestingModule } from '@angular/router/testing';
import { AppComponent } from './app.component';
import { AppMainComponent } from './app.main.component';
import { AppTopBarComponent } from './app.topbar.component';
import { AppFooterComponent } from './app.footer.component';
import { AppMenuComponent } from './app.menu.component';
import { ScrollPanelModule } from 'primeng/scrollpanel';
import { CalendarModule } from 'primeng/calendar';
import { TabViewModule } from 'primeng/tabview';
import { CheckboxModule } from 'primeng/checkbox';
import { AppConfigComponent } from './app.config.component';
import { AppRightPanelComponent } from './app.rightpanel.component';
import { AppBreadcrumbComponent } from './app.breadcrumb.component';
import { BreadcrumbService } from './breadcrumb.service';

describe('AppComponent', () => {
    beforeEach(() => {
        TestBed.configureTestingModule({
            imports: [RouterTestingModule, ScrollPanelModule, CalendarModule, TabViewModule, CheckboxModule],
            declarations: [
                AppComponent,
                AppMainComponent,
                AppMenuComponent,
                AppTopBarComponent,
                AppConfigComponent,
                AppRightPanelComponent,
                AppFooterComponent,
                AppBreadcrumbComponent
            ],
            providers: [BreadcrumbService]
        });
        TestBed.compileComponents();
    });

    it('should create the app', async(() => {
        const fixture = TestBed.createComponent(AppComponent);
        const app = fixture.debugElement.componentInstance;
        expect(app).toBeTruthy();
    }));
});
