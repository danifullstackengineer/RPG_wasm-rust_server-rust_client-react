import React, { useCallback, useEffect, useRef, useState } from 'react'
import styles from '../../styles/parent/MainForm.module.scss';
import MainButton from '../child/MainButton';



const MainForm = ({selectedButton}: {selectedButton: number}) => {


  // Handle switching between the 3 options


  return (
    <div className={styles.mainForm}>
      <MainButton text={"Log In"} link={"/login"} selected={selectedButton === 0}/>
      <MainButton text={"Register"} link={"/register"} selected={selectedButton === 1}/>
      <MainButton text={"Docs"} link={"/docs"} selected={selectedButton === 2}/>
    </div>
  )
}

export default React.memo(MainForm);