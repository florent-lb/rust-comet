import {Component} from '@angular/core';
import {AccountService} from "app/service/account.service";

@Component({
    selector: 'app-init-data-button',
    standalone: true,
    imports: [],
    template: `
        <button (click)="this.accountService.initialize()">Init accounts</button>
    `
})
export class InitDataButtonComponent {

    constructor(public readonly accountService: AccountService) {
    }
}
