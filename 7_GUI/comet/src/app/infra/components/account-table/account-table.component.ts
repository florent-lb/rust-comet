import { Component } from '@angular/core';
import {AccountService} from "../../../service/account.service";
import {AmountSensPipe} from "../../pipe/amount-sens.pipe";
import {
  MatCell,
  MatCellDef,
  MatColumnDef,
  MatHeaderCell,
  MatHeaderCellDef, MatHeaderRow,
  MatHeaderRowDef, MatRow, MatRowDef,
  MatTable
} from "@angular/material/table";
import {NoDataRowOutlet} from "@angular/cdk/table";

@Component({
  selector: 'app-account-table',
  standalone: true,
  imports: [
    AmountSensPipe,
    MatTable,
    MatColumnDef,
    MatHeaderCell,
    MatCell,
    MatHeaderCellDef,
    MatCellDef,
    MatHeaderRowDef,
    MatRowDef,
    MatHeaderRow,
    MatRow,
    NoDataRowOutlet
  ],
  templateUrl: './account-table.component.html',
  styleUrl: './account-table.component.scss'
})
export class AccountTableComponent {
  displayedColumns: string[] = ['number', 'amount'];

  constructor(public readonly accountService: AccountService) {
  }

}
