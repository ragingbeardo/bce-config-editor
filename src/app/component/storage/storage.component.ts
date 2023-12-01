import { Component } from '@angular/core';
import { SharedModule } from 'src/app/shared/shared.module';

@Component({
  selector: 'app-storage',
  standalone: true,
  imports: [SharedModule],
  templateUrl: './storage.component.html',
  styleUrls: ['./storage.component.css', '../../shared/shared.css']
})
export class StorageComponent {

}
