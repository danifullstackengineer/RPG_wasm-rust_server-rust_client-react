import React, { useEffect, useRef, useState } from 'react'
import styles from '../../styles/single/Experience.module.scss';
import effect from '../../assets/bitmaps/misc/effect_nobg.gif'

const Experience = () => {

  const [showImg, setShowImg] = useState<boolean>(false);
  const amountRef = useRef<HTMLDivElement>(null);

  const [currentEXP, setCurrentEXP] = useState<number>(500);
  const [maxEXP, setMaxEXP] = useState<number>(1000);

  useEffect(()=>{
    if(amountRef.current){
      amountRef.current.style.width = ((currentEXP / maxEXP) * 100) + "%";
    }
  }, [currentEXP, maxEXP, amountRef])

  return (
    <div className={styles.experience}>
        {showImg  ? <img src={effect}/> : ""} 
        <span className={styles.experience__level}>Level: 0</span>
        <div className={styles.experience__amount} ref={amountRef}></div>
        <div className={styles.experience__counter}>
          <span className={styles.experience__counter__current}>1003223209</span>
          <span>/</span>
          <span className={styles.experience__counter__max}>1003223209</span>
        </div>
    </div>
  )
}

export default Experience