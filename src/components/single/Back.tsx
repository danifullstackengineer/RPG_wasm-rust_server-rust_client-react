import React from 'react'
import styles from '../../styles/single/Back.module.scss';
import {GiReturnArrow} from 'react-icons/gi';
import { useNavigate } from 'react-router';
import EventListener from 'react-event-listener';


const Back = () => {

    const navigate = useNavigate();

    const handleNavigate = () => {
        navigate(-1);
    }

    const handleKeyDown = (ev:KeyboardEvent):void => {
      if(ev.key === "Escape"){
        handleNavigate();
      }
    }

  return (
    <>
    <EventListener target={"document"} onKeyDown={(e) => handleKeyDown(e)}/>
    <div className={styles.back} onClick={handleNavigate}>
        <GiReturnArrow style={{width: 50, height: 50}}/>
    </div>
    </>
  )
}

export default React.memo(Back);