import React, { useEffect, useRef, useState } from "react";
import styles from "../../styles/grandparent/Register.module.scss";
import char from "../../assets/bitmaps/player/Hero Knight/Sprites/HeroKnight/Idle/gif.gif";
import { MdPlayArrow } from "react-icons/md";
import EventListener from "react-event-listener";
import {fetch_classes} from 'rust';
import { useContext } from "react";
import WasmContext from "../../assets/context/WASM";

const Register = () => {

  const wasmContext = useContext(WasmContext);

  const [email, setEmail] = useState<string>("");
  const [password, setPassword] = useState<string>("");
  const [repassword, setRepassword] = useState<string>("");

  const emailRef = useRef<HTMLInputElement>(null);
  const passwordRef = useRef<HTMLInputElement>(null);
  const repasswordRef = useRef<HTMLInputElement>(null);

  const [selected, setSelected] = useState<number>(0);

  const [loading, setLoading] = useState<boolean>(false);

  const [warning, setWarning] = useState<string>("This is a warning.");
  const [showWarn, setShowWarn] = useState<boolean>(false);

  const [isActiveBtn, setIsActiveBtn] = useState<number>(0);

  const [chosenClass, setChosenClass] = useState<number>(0);

  useEffect(()=>{
    if(wasmContext.isInit){
      fetch_classes().then(res=>{
        // TODO: Finish this implementation.
        console.log(res);
      })
    }
  }, [wasmContext])

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

  const handleKeyDown = (e: KeyboardEvent): void => {
    if(e.key === 'ArrowUp'){
        if(selected === 1 && emailRef.current) {
            emailRef.current.focus();
            setSelected(0);
        }
        else if(selected === 2 && passwordRef.current){
            passwordRef.current.focus();
            setSelected(1);
        }
    }else if(e.key === 'ArrowDown'){
        if(selected === 0 && passwordRef.current){
            passwordRef.current.focus();
            setSelected(1);
        }else if(selected === 1 && repasswordRef.current){
            repasswordRef.current.focus();
            setSelected(2);
        }
    }
  };

  const handleSubmit = (e: React.FormEvent<HTMLFormElement>): void => {
    e.preventDefault();
    if(wasmContext.isInit){
      // verify_and_register(email, password, repassword, chosenClass)
    }
  };


  return (
      <>
      <EventListener target={"document"} onKeyDown={(ev) => handleKeyDown(ev)}/>
    <div className={styles.register}>
      <div className={styles.register__container}>
        <div className={styles.register__container__left}>
          <button type="button">Previous</button>
          <img src={char} alt={""} loading={"eager"} />
          <button type="button">Next</button>
        </div>
        <div className={styles.register__container__right}>
          <h2>Knight</h2>
          <h4>
            <span>Description</span>
            <span>
              A narrow crypt in a somber thicket marks the entrance to this
              dungeon. Beyond the crypt lies a scanty, dusty room. It's covered
              in broken stone, broken pottery and broken stone. Your torch
              allows you to see empty chests and broken statues, weathered and
              dismantled by time itself. Further ahead are two paths, you take
              the left. Its twisted trail leads passed lost treasuries, unknown
              rooms and armories and soon you enter a dusty area. There's a
              demolished door with a sign that says "don't open". What happened
              in this place? You tread onwards, deeper into the dungeon's
              expanse. You pass dozens of similar rooms and passages, some are
              dead ends, others lead to who knows where, or what. You eventually
              make it to what is likely the final room. A huge metal door blocks
              your path. Intricate carvings are all over it, somehow untouched
              by time and the elements. You step closer to inspect it and..
              wait.. you hear the faint sound of footsteps behind you.
            </span>
          </h4>
          <div className={styles.register__container__left__container}>
            <div className={styles.register__container__right__stats}>
              <div className={styles.register__stats__name}>
                <span>Physical Attack</span>
                <span>Magical Attack</span>
                <span>Speed</span>
                <span>Defense</span>
                <span>Health</span>
              </div>
              <div className={styles.register__stats__number}>
                <span>6</span>
                <span>0</span>
                <span>1</span>
                <span>4</span>
                <span>4</span>
              </div>
            </div>
            <div className={styles.register__container__right__form}>
              <form onSubmit={(e) => handleSubmit(e)}>
                <span>Email</span>
                <input
                  type="email"
                  value={email}
                  onChange={(e) => setEmail(e.target.value)}
                  required={true}
                  placeholder={"A valid email address..."}
                  autoFocus={true}
                  ref={emailRef}
                  onClick={() => setSelected(0)}
                />
                <span>Password</span>
                <input
                  className={styles.register__register__pw}
                  type="text"
                  value={password}
                  onChange={(e) => setPassword(e.target.value)}
                  required={true}
                  placeholder={"A valid password..."}
                  ref={passwordRef}
                  onClick={() => setSelected(1)}
                />
                <span>Email</span>
                <input
                  className={styles.register__register__pw}
                  type="text"
                  value={repassword}
                  onChange={(e) => setRepassword(e.target.value)}
                  required={true}
                  placeholder={"Repeat your password..."}
                  ref={repasswordRef}
                  onClick={() => setSelected(2)}
                />
                <button
                  className={loading ? styles.register__btn__loading : ""}
                  type="submit"
                >
                  {loading ? (
                    <>
                      <div
                        style={{
                          backgroundColor:
                            isActiveBtn === 0 ? "red" : undefined,
                        }}
                      ></div>
                      <div
                        style={{
                          backgroundColor:
                            isActiveBtn === 1 ? "red" : undefined,
                        }}
                      ></div>
                      <div
                        style={{
                          backgroundColor:
                            isActiveBtn === 2 ? "red" : undefined,
                        }}
                      ></div>
                    </>
                  ) : (
                    "Register"
                  )}
                </button>
                <span
                  style={{ color: "red", opacity: !showWarn ? "0" : undefined }}
                >
                  {warning}
                </span>
              </form>
            </div>
          </div>
        </div>
      </div>
    </div>
    </>
  );
};

export default React.memo(Register);
