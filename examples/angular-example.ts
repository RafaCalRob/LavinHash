// Angular Example for LavinHash
// Works with Angular CLI (uses Webpack internally)

import { Component } from '@angular/core';
import { wasm_compare_data, wasm_generate_hash } from 'lavinhash';

@Component({
  selector: 'app-similarity-checker',
  template: `
    <div class="similarity-checker">
      <h2>Text Similarity Checker</h2>

      <div>
        <label>Text 1:</label>
        <textarea
          [(ngModel)]="text1"
          (input)="calculateSimilarity()"
          rows="3">
        </textarea>
      </div>

      <div>
        <label>Text 2:</label>
        <textarea
          [(ngModel)]="text2"
          (input)="calculateSimilarity()"
          rows="3">
        </textarea>
      </div>

      <div>
        <h3>Similarity: {{ similarity }}%</h3>
      </div>

      <button (click)="calculateSimilarity()">Recalculate</button>
    </div>
  `
})
export class SimilarityCheckerComponent {
  text1 = 'The quick brown fox jumps over the lazy dog';
  text2 = 'The quick brown fox leaps over the lazy dog';
  similarity: number | null = null;

  ngOnInit() {
    this.calculateSimilarity();
  }

  calculateSimilarity() {
    const encoder = new TextEncoder();
    const data1 = encoder.encode(this.text1);
    const data2 = encoder.encode(this.text2);

    this.similarity = wasm_compare_data(data1, data2);
  }

  // Example: Generate and compare hashes
  async compareFiles(file1: File, file2: File) {
    const buffer1 = await file1.arrayBuffer();
    const buffer2 = await file2.arrayBuffer();

    const data1 = new Uint8Array(buffer1);
    const data2 = new Uint8Array(buffer2);

    return wasm_compare_data(data1, data2);
  }
}
