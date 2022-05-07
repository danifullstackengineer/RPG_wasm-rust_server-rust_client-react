import React, { createContext } from "react";

type WasmContextType = {
  isInit: boolean;
  isInitShared: boolean;
  setIsInit: React.Dispatch<React.SetStateAction<boolean>>;
  setIsInitShared: React.Dispatch<React.SetStateAction<boolean>>;
};
const WasmContext = createContext<WasmContextType>({
  isInit: false,
  isInitShared: false,
  setIsInit: () => {},
  setIsInitShared: () => {}
});

export type { WasmContextType };
export default WasmContext;
