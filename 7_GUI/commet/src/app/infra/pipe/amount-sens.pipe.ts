import {Pipe, PipeTransform} from '@angular/core';

@Pipe({
    name: 'amountSens',
    standalone: true
})
export class AmountSensPipe implements PipeTransform {

    transform(value: 'D' | 'C'): '-' | '+' {
        return value === 'D' ? '-' : '+'
    }

}
