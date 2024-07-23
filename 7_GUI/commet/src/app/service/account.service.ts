import {Injectable, signal} from '@angular/core';
import {invoke} from "@tauri-apps/api/tauri";

@Injectable({
    providedIn: 'root'
})
export class AccountService {

    dataInitialize = signal<boolean>(false)

    constructor() {
    }


    initialize(): void {
        invoke<string>("init_accounts").then(() => {
            this.dataInitialize.set(true);
        });
    }

}
