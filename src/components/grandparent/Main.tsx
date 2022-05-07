import React, { useEffect, useState } from 'react';
import styles from '../../styles/grandparent/Main.module.scss';
import MainControls from '../parent/MainControls';
import MainForm from '../parent/MainForm';
import EventListener, {withOptions} from 'react-event-listener';
import {handle_key_switch, handle_enter_switch} from 'rust';
import {Keys} from 'r_shared';
import { useNavigate } from 'react-router';

const Main = () => {


  const navigate = useNavigate();

  const [selectedButton, setSelectedButton] = useState<number>(0);

  const handleKeyDown = (ev:KeyboardEvent) => {
    if(ev.key === 'ArrowUp'){
      setSelectedButton(handle_key_switch(selectedButton, Keys.ArrowUp));
    }else if(ev.key === 'ArrowDown'){
      setSelectedButton(handle_key_switch(selectedButton, Keys.ArrowDown));
    }
    else if(ev.key === 'Enter'){
      navigate(handle_enter_switch(selectedButton));
    }
  }

  return (
    <div className={styles.main}>
      <EventListener target={"document"} onKeyDown={(ev) => handleKeyDown(ev)}/>
        <MainForm selectedButton={selectedButton}/>
      <MainControls/>
    </div>
  )

}

export default React.memo(Main)