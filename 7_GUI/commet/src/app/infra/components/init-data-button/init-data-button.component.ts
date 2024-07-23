import {Component} from '@angular/core';
import {AccountService} from "../../../service/account.service";
import {MatButton} from "@angular/material/button";

@Component({
    selector: 'app-init-data-button',
    standalone: true,
    imports: [
        MatButton
    ],
    styles: `
    button {
      margin : 3px;
      margin-bottom: 3vh;
      margin-top: 3vh;
    }
    `
    ,
    template: `
        <button mat-flat-button [disabled]="this.accountService.dataInitialize()"
                (click)="this.accountService.initialize()">Init accounts</button>
    `
})
export class InitDataButtonComponent {

    constructor(public readonly accountService: AccountService) {
    }
}
