import React, { useEffect, useState } from "react";
import init from "rust";
import { default as init_shared } from "r_shared";
import "./styles/index.css";
import { BrowserRouter as Router, Routes, Route } from "react-router-dom";
import Main from "./components/grandparent/Main";
import Login from "./components/grandparent/Login";
import Player from "./components/single/Player";
import Back from "./components/single/Back";
import Register from "./components/grandparent/Register";
import WasmContext from "./assets/context/WASM";
import { WasmContextType } from "./assets/context/WASM";
import Home from "./components/grandparent/Home";
import Experience from "./components/single/Experience";
import HealthBar from "./components/single/HealthBar";

function App() {
  const [isInit, setIsInit] = useState<boolean>(false);
  const [isInitShared, setIsInitShared] = useState<boolean>(false);

  useEffect(() => {
    init().then(() => {
      setIsInit(true);
    });
    init_shared().then(()=> {
      setIsInitShared(true);
    })
  }, []);


  // Event listeners:

  return (
    <div className="app">
      <WasmContext.Provider
        value={{
          isInit: isInit,
          isInitShared: isInitShared,
          setIsInit: setIsInit,
          setIsInitShared: setIsInitShared,
        }}
      >
        <Router>
          <Routes>
            <Route
              path="/*"
              element={
                <>
                  <Player />
                  <Main />
                </>
              }
            />
            <Route
              path="/login"
              element={
                <>
                  <Back />
                  <Player />
                  <Login />
                </>
              }
            />
            <Route
              path="/register"
              element={
                <>
                  <Back />
                  <Player />
                  <Register />
                </>
              }
            />
            <Route
              path="/home"
              element={
                <>
                  <Back />
                  <Player />
                  <Experience />
                  <HealthBar />
                  <Home />
                </>
              }
            />
          </Routes>
        </Router>
      </WasmContext.Provider>
    </div>
  );
}

export default App;
