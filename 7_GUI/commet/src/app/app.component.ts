import { Component } from '@angular/core';
import { CommonModule } from '@angular/common';
import { RouterOutlet } from '@angular/router';
import { InitDataButtonComponent } from "app/infra/components/init-data-button/init-data-button.component";

@Component({
  selector: 'app-root',
  standalone: true,
  imports: [CommonModule, RouterOutlet, InitDataButtonComponent, InitDataButtonComponent],
  template: `
  <app-init-data-button></app-init-data-button>
  `

})
export class AppComponent {

}
