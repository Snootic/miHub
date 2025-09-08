import React, {useState } from "react";
import { Button, Input, Select } from "../../components";
import { useCheckinForm, FormData } from "../../hooks/dailyCheckinForm";
import { AppContextProvider } from "../../components/context/contextProvider";

import './index.css'

const CheckinForm: React.FC = () => {
  const [mode, setMode] = useState<string>("daily_checkin");
  const [game, setGame] = useState<string>("nap");
  const [account, setAccount] = useState<string>("");
  const [password, setPassword] = useState<string>("");
  const [hoyolabId, setHoyolabId] = useState<string>("");
  const [cookies, setCookies] = useState<string>("");
  const [uid, setUid] = useState<string>("");
  const [code, setCode] = useState<string>("");
  const [isSubmitting, setIsSubmitting] = useState<boolean>(false);

  const { validateFields, submitForm } = useCheckinForm();

  // Get field validation state
  const validation = validateFields(account, password, hoyolabId, cookies);

  const handleSubmit = async (e: React.FormEvent) => {
    e.preventDefault();
    
    if (!validation.isValid) {
      alert("Please fill in the required authentication fields");
      return;
    }

    if (mode === "redeem_promo_code" && !code) {
      alert("Code is required for promo code redemption");
      return;
    }

    setIsSubmitting(true);
    
    const formData: FormData = {
      mode,
      game,
      account: account || undefined,
      password: password || undefined,
      hoyolab_id: hoyolabId || undefined,
      cookies: cookies || undefined,
      uid: uid || undefined,
      code: code || undefined,
    };

    const result = await submitForm(formData);
    
    if (result?.success) {
      alert("Form submitted successfully!");
    } else {
      alert(`Error: ${result?.error?.message || "Unknown error"}`);
    }
    
    setIsSubmitting(false);
  };

  return (
    <AppContextProvider>
      <div className="container">
        <form id="hoyolabForm" onSubmit={handleSubmit}>
          <label htmlFor="mode">Modo:</label>
          <Select
            options={["daily_checkin", "redeem_promo_code"]}
            value={mode}
            setValue={setMode}
          />

          <label htmlFor="game">Jogo:</label>
          <Select
            options={["nap", "genshin", "hkrpg"]}
            value={game}
            setValue={setGame}
            />

          <label htmlFor="account">Conta:</label>
          <Input
            placeholder="Conta"
            value={account}
            onChange={(e) => setAccount(e.target.value)}
            disabled={validation.accountDisabled}
            />

          <label htmlFor="password">Senha:</label>
          <Input
            placeholder="Senha"
            value={password}
            onChange={(e) => setPassword(e.target.value)}
            type="password"
            disabled={validation.passwordDisabled}
          />

          <label htmlFor="hoyolab_id">Hoyolab ID:</label>
          <Input
            placeholder="Hoyolab ID"
            value={hoyolabId}
            onChange={(e) => setHoyolabId(e.target.value)}
            disabled={validation.hoyolabIdDisabled}
            />

          <label htmlFor="cookies">Cookies:</label>
          <Input
            placeholder="Cookies"
            value={cookies}
            onChange={(e) => setCookies(e.target.value)}
            disabled={validation.cookiesDisabled}
            />

          {mode === "redeem_promo_code" && (
            <div id="promoFields">
              <label htmlFor="uid">UID:</label>
              <Input
                placeholder="UID"
                value={uid}
                onChange={(e) => setUid(e.target.value)}
                />

              <label htmlFor="code">Código:</label>
              <Input
                placeholder="Código"
                value={code}
                onChange={(e) => setCode(e.target.value)}
                />
            </div>
          )}

          <Button
            variant="primary"
            type="submit"
            disabled={isSubmitting || !validation.isValid}
            text={isSubmitting ? "Enviando..." : "Enviar"}
            style={{width: "200px"}}
            />
        </form>
      </div>
    </AppContextProvider>
  );
};

export default CheckinForm;