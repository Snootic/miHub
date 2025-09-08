import { invoke } from '@tauri-apps/api/core';
import { useEffect, useCallback } from 'react';

export interface FormData {
  mode: string;
  game: string;
  account?: string;
  password?: string;
  hoyolab_id?: string;
  cookies?: string;
  uid?: string;
  code?: string;
}

export const useCheckinForm = () => {
  
  const runTasksAsync = useCallback(async () => {
    console.log("running tasks");

    try {
      await invoke('run_checkin');
      await invoke('run_code_redeem');
    } catch (error) {
      console.error("Error running tasks:", error);
    }

    let timeout = 600000 * 60 * 3; // 3 hours
    console.log(timeout);

    setTimeout(runTasksAsync, timeout);
  }, []);

  const validateFields = useCallback((
    account: string,
    password: string,
    hoyolab_id: string,
    cookies: string
  ) => {
    const validation = {
      accountDisabled: false,
      passwordDisabled: false,
      hoyolabIdDisabled: false,
      cookiesDisabled: false,
      isValid: false
    };

    if (account || password) {
      validation.hoyolabIdDisabled = true;
      validation.cookiesDisabled = true;
      validation.isValid = !!(account && password);
    }
    else if (hoyolab_id) {
      validation.accountDisabled = true;
      validation.passwordDisabled = true;
      validation.cookiesDisabled = true;
      validation.isValid = true;
    }
    else if (cookies) {
      validation.accountDisabled = true;
      validation.passwordDisabled = true;
      validation.hoyolabIdDisabled = true;
      validation.isValid = true;
    }
    else {
      validation.isValid = false;
    }

    return validation;
  }, []);

  const submitForm = useCallback(async (formData: FormData) => {
    try {
      const {
        mode,
        game,
        account,
        password,
        hoyolab_id,
        cookies,
        uid,
        code
      } = formData;

      const data: Record<string, string> = {};
      if (game) data.game = game;
      if (account) data.account = account;
      if (password) data.password = password;
      if (hoyolab_id) data.hoyolab_id = hoyolab_id;
      if (cookies) data.cookies = cookies;

      if (mode === 'redeem_promo_code') {
        if (uid) data.uid = uid;
        console.log('UID:', data.uid);
        
        if (code) {
          await invoke(mode, { mode: mode, data: data });
          return;
        } else {
          throw new Error('Code is required for promo code redemption');
        }
      }

      await invoke('save_data', { mode: mode, data: data });
      await invoke(mode, { args: data });
      
      console.log('Form submitted successfully');
      return { success: true };
      
    } catch (error) {
      console.error('Error submitting form:', error);
      return { success: false, error: error as Error };
    }
  }, []);

  useEffect(() => {
    runTasksAsync();
  }, [runTasksAsync]);

  return {
    validateFields,
    submitForm,
    runTasksAsync
  };
};
