// src/app/components/share-contract/share-contract.component.ts
import { JsonPipe } from '@angular/common';
import { Component, computed, inject, OnInit, signal } from '@angular/core';
import { FormBuilder, FormGroup, ReactiveFormsModule, Validators } from '@angular/forms';
import { Router } from '@angular/router';
import { Contract, ShareRequest } from '../../models/contract.model';
import { User, UserUtils } from '../../models/user.model';
import { ContractService } from '../../services/contract.service';
import { UserService } from '../../services/user.service';

@Component({
  selector: 'app-share-contract',
  standalone: true,
  imports: [JsonPipe, ReactiveFormsModule],
  templateUrl: './share-contract.component.html',
  styleUrl: './share-contract.component.scss'
})
export class ShareContractComponent implements OnInit {
  private userService = inject(UserService);
  private contractService = inject(ContractService);
  shareForm: FormGroup;
  contracts = signal<Contract[]>([]);
  loading = signal<boolean>(false);
  sharing = signal<boolean>(false);
  error = signal<string | null>(null);
  success = signal<string | null>(null);
  currentUser = computed<User | null>(() => this.userService.currentUser());
  canShare = signal<boolean>(false);

  // ✅ NOVO: Métodos para trabalhar com roles
  getUserRoleDescription(): string {
    const user = this.currentUser();
    return user ? UserUtils.getRoleDescription(user) : 'Guest';
  }

  getUserBadgeClass(): string {
    const user = this.currentUser();
    if (!user) {
      return 'user-guest';
    }
    
    if (UserUtils.hasMultipleRoles(user)) {
      return 'user-multiple';
    }
    
    if (UserUtils.isClient(user)) {
      return 'user-client';
    }
    
    if (UserUtils.isPartner(user)) {
      return 'user-partner';
    }
    
    return 'user-unknown';
  }

  // ✅ NOVO: Função helper para truncar texto da combobox
  getTruncatedContractText(contract: Contract, maxLength: number = 45): string {
    const text = `${contract.title} (ID: ${contract.id})`;
    // Use Intl.Segmenter to split into grapheme clusters
    const segmenter = typeof Intl !== 'undefined' && 'Segmenter' in Intl
      ? new Intl.Segmenter(undefined, { granularity: 'grapheme' })
      : null;

    if (!segmenter) {
      // Fallback to original logic if Segmenter is not available
      if (text.length <= maxLength) {
        return text;
      }
      return text.substring(0, maxLength - 3) + '...';
    }

    const graphemes = Array.from(segmenter.segment(text), seg => seg.segment);
    if (graphemes.length <= maxLength) {
      return text;
    }
    return graphemes.slice(0, maxLength - 3).join('') + '...';
  }

  constructor(
    private fb: FormBuilder,
    private router: Router
  ) {
    this.shareForm = this.fb.group({
      process_id: ['', [Validators.required]],
      partner_public_key: ['', [
        Validators.required,
        Validators.minLength(56),
        Validators.maxLength(56),
        Validators.pattern(/^G[A-Z0-9]{55}$/) // Stellar public key pattern
      ]],
      client_username: [{ value: '', disabled: true }]
    });
  }

  ngOnInit() {
    this.checkUserPermissions();
    
    if (this.canShare()) {
      this.loadContracts();
    }
  }

  // ✅ NOVO: Verificar permissões do usuário
  checkUserPermissions() {
    const user = this.userService.currentUser();
    
    console.log('👤 Current user:', user);
    
    if (!user) {
      this.error.set('User not logged in');
      return;
    }

    this.contractService.getPermissions().subscribe({
      next: permissions => {
        this.canShare.set(permissions.canShare);
        console.log('🔐 Can share contracts:', this.canShare());

        if (!this.canShare()) {
          this.error.set('Only clients can share contracts. You are logged in as a partner  .');
        }

        // Preencher username no formulário
        this.shareForm.patchValue({
          client_username: user.username
        });
      },
      error: err => {
        console.error('❌ Error fetching permissions:', err);
        this.error.set('Error fetching user permissions. Please try again.');
      }
    });
  }

  loadContracts() {
    this.loading.set(true);
    this.error.set(null);

    const user = this.currentUser();
    if (!user) {
      this.error.set('User not found');
      this.loading.set(false);
      return;
    }

    this.contractService.listContracts().subscribe({
      next: (contracts) => {
        this.contracts.set(contracts);
        this.loading.set(false);
        
        if (contracts.length === 0) {
          this.error.set('No contracts found. Create a contract first before sharing.');
        }
      },
      error: (err) => {
        console.error('❌ Error loading contracts:', err);
        this.error.set('Error loading contracts. Please try again.');
        this.loading.set(false);
      }
    });
  }

  onSubmit() {
    if (!this.canShare()) {
      this.error.set('You do not have permission to share contracts.');
      return;
    }

    if (this.shareForm.valid) {
      this.sharing.set(true);
      this.error.set(null);
      this.success.set(null);

      const user = this.currentUser();
      if (!user) {
        this.error.set('User not found');
        this.sharing.set(false);
        return;
      }

      const shareData: ShareRequest = {
        process_id: this.shareForm.get('process_id')?.value,
        partner_public_key: this.shareForm.get('partner_public_key')?.value,
        client_username: user.username
      };

      console.log('🔗 Submitting share request:', shareData);

      this.contractService.shareContract(shareData).subscribe({
        next: (response) => {
          console.log('✅ Share successful:', response);
          this.success.set('Contract shared successfully with partner!');
          this.sharing.set(false);
          this.shareForm.reset();
          this.shareForm.patchValue({ client_username: user.username });
        },
        error: (err) => {
          console.error('❌ Share failed:', err);
          this.error.set(err.error?.message || 'Failed to share contract. Please try again.');
          this.sharing.set(false);
        }
      });
    } else {
      this.markFormGroupTouched();
    }
  }

  private markFormGroupTouched() {
    Object.keys(this.shareForm.controls).forEach(key => {
      const control = this.shareForm.get(key);
      control?.markAsTouched();
    });
  }

  goBack() {
    this.router.navigate(['/contracts']);
  }

  isFieldInvalid(fieldName: string): boolean {
    const field = this.shareForm.get(fieldName);
    return !!(field && field.invalid && field.touched);
  }

  getFieldError(fieldName: string): string {
    const field = this.shareForm.get(fieldName);
    if (field?.errors) {
      if (field.errors['required']) {
        return `${fieldName} is required`;
      }
      if (field.errors['minlength']) {
        return `${fieldName} must be at least ${field.errors['minlength'].requiredLength} characters`;
      }
      if (field.errors['maxlength']) {
        return `${fieldName} must be at most ${field.errors['maxlength'].requiredLength} characters`;
      }
      if (field.errors['pattern']) {
        return `${fieldName} format is invalid`;
      }
    }
    return '';
  }

  validateStellarPublicKey(key: string): boolean {
    return /^G[A-Z0-9]{55}$/.test(key);
  }

  onPublicKeyInput(event: Event) {
    const input = event.target as HTMLInputElement;
    const value = input.value.toUpperCase();
    this.shareForm.patchValue({ partner_public_key: value });
  }
}