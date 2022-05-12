import 'regenerator-runtime/runtime'
import { useEffect, useState } from 'react'
import ListResources from './components/ListResources.js'
import AddResource from './components/AddResources.js'
import pic from "./images/near_logo.png";
import React from 'react'
import { login, logout } from './utils'
import './global.css'

import getConfig from './config'

const { networkId } = getConfig(process.env.NODE_ENV || 'development')

export default function App() {

  const [events, setEvents] = useState([])
  const [toggleModal, setToggleModal] = useState(false)


  function addProject() {
    setToggleModal(!toggleModal)
  }

  
  useEffect(
    () => {

      if (window.walletConnection.isSignedIn()) {

        window.contract.list_resources().then((eventprojects) => {
          const eventList = [...eventprojects]
          setEvents(eventList)
        })
      }
    },

    [],
  )

  if (!window.walletConnection.isSignedIn()) {
    return (
      <main className='signin'>
        <h1>Near Resources</h1> 
        <p style={{ textAlign: 'center' }}>
          One stop for all resources you need to develop with Near.
        </p>
        <p style={{ textAlign: 'center' }}>
          Click the button below to login:
        </p>
        <p style={{ textAlign: 'center', marginTop: '2.5em' }}>
          <button onClick={login}>Login</button>
        </p>
      </main>
    )
  }

  return (

    <>
      <header>
        <div className="logo"></div>
        <img className="logo-pic" src={pic} />
        <button className="link" style={{ float: 'right' }} onClick={logout}>
          Sign out <span className="id">{window.accountId}</span>
        </button>
      </header>
      <button onClick={addProject}>Add a resource</button>
      <main className="resource-list">
        <p>
          List of resources:
        </p>
        <AddResource toggleModal={toggleModal} />
        <section className="events">
          {events.map((project, id) => {
            return (
              <div className="resource" key={id}>
                <ListResources project={project} />
              </div>
            )
          })}
        </section>
      </main>
    </>
  )
}