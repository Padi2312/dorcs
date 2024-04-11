// store.js
import { writable } from 'svelte/store';
import type { PageSettings } from './types';

export const pageSettings = writable<PageSettings>();

export const fetchPageSettings = async () => {
  try {
    const response = await fetch("/page_settings.json");
    const data = await response.json();
    pageSettings.set(data);
  } catch (error) {
    console.error('Failed to fetch data:', error);
  }
};