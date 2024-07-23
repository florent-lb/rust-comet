import { Component } from '@angular/core';

@Component({
  selector: 'app-footer',
  standalone: true,
  imports: [],
  styles:`
  :host{
    background: #333333!important;
    color: white!important;
  }
  `,
  template: `
  <span>powered by Extia</span>`
})
export class FooterComponent {

}
