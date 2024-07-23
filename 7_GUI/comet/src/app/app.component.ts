import { Component } from '@angular/core';
import { CommonModule } from '@angular/common';
import { RouterOutlet } from '@angular/router';
import { InitDataButtonComponent } from "./infra/components/init-data-button/init-data-button.component";
import {MenuExtiaComponent} from "./infra/components/menu-extia/menu-extia.component";
import {MatDivider} from "@angular/material/divider";
import {FooterComponent} from "./infra/components/footer/footer.component";
import {AccountTableComponent} from "./infra/components/account-table/account-table.component";

@Component({
  selector: 'app-root',
  standalone: true,
  imports: [CommonModule, RouterOutlet, InitDataButtonComponent, InitDataButtonComponent, MenuExtiaComponent, MatDivider, FooterComponent, AccountTableComponent],
  styles: `
  .grid-container {
    height: 100%;
    display: grid;
    grid-template-rows: 1fr 10fr 1fr;
  }
  `,
  template: `
      <div class="grid-container">
          <div class="grid-header">
              <app-menu-extia></app-menu-extia>
              <mat-divider></mat-divider>
          </div>
          <div class="content">
              <app-init-data-button></app-init-data-button>
              <mat-divider></mat-divider>
              <app-account-table></app-account-table>
          </div>
          <app-footer></app-footer>
      </div>
  `

})
export class AppComponent {

}
