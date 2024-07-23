import {computed, effect, Injectable, signal} from '@angular/core';
import {invoke} from "@tauri-apps/api/tauri";
import {Account} from "../domain/account.model";

@Injectable({
    providedIn: 'root'
})
export class AccountService {

    dataInitialize = signal<boolean>(false)
    accounts = signal<Account[]>([])

    constructor() {
        effect(() => {
            if (this.dataInitialize()) {
                this.reloadAccounts();
            }
        });
    }


    initialize(): void {
        invoke<string>("init_accounts").then(() => {
            this.dataInitialize.set(true);
        });
    }

    private reloadAccounts() {
        invoke<Account[]>("get_all_accounts")
            .then((accounts) => this.accounts.set(accounts));
    }
}
