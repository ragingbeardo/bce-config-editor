import { Component } from '@angular/core';
import { SharedModule } from 'src/app/shared/shared.module';

@Component({
  selector: 'app-scav',
  standalone: true,
  imports: [SharedModule],
  templateUrl: './scav.component.html',
  styleUrls: ['./scav.component.css', '../../shared/shared.css']
})
export class ScavComponent {

}
