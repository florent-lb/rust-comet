import {Component} from '@angular/core';
import {AccountService} from "app/service/account.service";
import {MatButton} from "@angular/material/button";

@Component({
    selector: 'app-init-data-button',
    standalone: true,
    imports: [
        MatButton
    ],
    template: `
        <button mat-flat-button [disabled]="this.accountService.dataInitialize()"
                (click)="this.accountService.initialize()">Init accounts</button>
    `
})
export class InitDataButtonComponent {

    constructor(public readonly accountService: AccountService) {
    }
}
