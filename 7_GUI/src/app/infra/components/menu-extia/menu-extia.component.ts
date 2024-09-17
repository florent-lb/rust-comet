import { Component } from '@angular/core';
import {MatToolbar} from "@angular/material/toolbar";
import {MatIconButton} from "@angular/material/button";
import {MatIcon} from "@angular/material/icon";

@Component({
  selector: 'app-menu-extia',
  standalone: true,
  imports: [
    MatToolbar,
    MatIconButton,
    MatIcon
  ],
  styles: `
    .example-spacer {
      flex: 1 1 auto;
    }
  `,
  template:`
  <mat-toolbar>
    <button mat-icon-button>
      <mat-icon>menu</mat-icon>
    </button>
    <span>Rust by Extia</span>
    <span class="example-spacer"></span>
    <button mat-icon-button class="favorite-icon">
      <mat-icon>favorite</mat-icon>
    </button>
    <button mat-icon-button  aria-label="Example icon-button with share icon">
      <mat-icon>share</mat-icon>
    </button>
  </mat-toolbar>
  `
})
export class MenuExtiaComponent {

}
