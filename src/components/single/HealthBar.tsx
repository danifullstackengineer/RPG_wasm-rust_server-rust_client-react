import React, { useEffect, useRef, useState } from 'react'
import styles from '../../styles/single/HealthBar.module.scss';

const HealthBar = () => {

  const [health, setHealth] = useState<number>(100);
  const healthRef = useRef<HTMLDivElement>(null);

  useEffect(()=>{
    if(healthRef.current){
      healthRef.current.style.transform = "translateY(" + (100 - health) / 1.5 + "%)";
    }
  },[health, healthRef])

  return (
    <div className={styles.healthBar}>
        <div className={styles.healthBar__container} ref={healthRef}>
        </div>
        <span>{health} %</span>
    </div>
  )
}

export default React.memo(HealthBar);