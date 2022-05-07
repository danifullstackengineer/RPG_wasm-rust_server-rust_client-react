import React, { useEffect, useRef, useState } from "react";
import EventListener from "react-event-listener";
import styles from "../../styles/grandparent/Login.module.scss";
import { useNavigate } from "react-router-dom";

const Login = () => {

  const navigate = useNavigate();

  const [email, setEmail] = useState<string>("");
  const [password, setPassword] = useState<string>("");
  const emailRef = useRef<HTMLInputElement>(null);
  const passwordRef = useRef<HTMLInputElement>(null);

  const [warning, setWarning] = useState<string>("This is a warning");
  const [showWarn, setShowWarn] = useState<boolean>(false);

  const [selected, setSelected] = useState<number>(0);

  const [loading, setLoading] = useState<boolean>(false);

  const handleKeyDown = (e: KeyboardEvent): void => {
    if (e.key === "ArrowUp" && selected === 1 && emailRef.current) {
      emailRef.current.focus();
      setSelected(0);
    } else if (e.key === "ArrowDown" && selected === 0 && passwordRef.current) {
      passwordRef.current.focus();
      setSelected(1);
    }
  };

  const [isActiveBtn, setIsActiveBtn] = useState<number>(0);

  useEffect(() => {
    if (loading) {
      const interval = setInterval(() => {
        setIsActiveBtn((prevState) =>
          prevState === 0 ? 1 : prevState === 1 ? 2 : prevState === 2 ? 0 : 0
        );
      }, 500);
      return () => clearInterval(interval);
    }
  }, [loading]);

  const handleSubmit = (e: React.FormEvent<HTMLFormElement>): void => {
    e.preventDefault();
    // navigate("/")
  };

  return (
    <>
      <EventListener
        target={"document"}
        onKeyDown={(ev) => handleKeyDown(ev)}
      />
      <div className={styles.login}>
        <form onSubmit={(ev) => handleSubmit(ev)}>
          <span>Email</span>
          <input
            type="email"
            value={email}
            onChange={(e) => setEmail(e.target.value)}
            placeholder={"A valid email address..."}
            required={true}
            autoFocus={true}
            ref={emailRef}
            onClick={() => setSelected(0)}
          />
          <span>Password</span>
          <input
            className={styles.login__input__pw}
            type="text"
            value={password}
            onChange={(e) => setPassword(e.target.value)}
            placeholder={"A valid password..."}
            required={true}
            ref={passwordRef}
            onClick={() => setSelected(1)}
          />
          <button
            type="submit"
            className={loading ? styles.login__btn__loading : ""}
          >
            {loading ? (
              <>
                <div
                  style={{
                    backgroundColor: isActiveBtn === 0 ? "red" : undefined,
                  }}
                ></div>
                <div
                  style={{
                    backgroundColor: isActiveBtn === 1 ? "red" : undefined,
                  }}
                ></div>
                <div
                  style={{
                    backgroundColor: isActiveBtn === 2 ? "red" : undefined,
                  }}
                ></div>
              </>
            ) : (
              "Log In"
            )}
          </button>
          <span style={{ color: "red", opacity: !showWarn ? "0" : undefined }}>
            {warning}
          </span>
        </form>
      </div>
    </>
  );
};

export default React.memo(Login);
