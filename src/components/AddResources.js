import React, { useState } from 'react'
    function AddResource({toggleModal}) {
      const [url, setUrl] = useState('')
      const [resource_type, setType] = useState('')
      const [target, setTarget] = useState(0)
      const [description, setDescription] = useState('')
      const [showNotification, setShowNotification] = useState(false)
      const handleSubmit = (event) => {
        event.preventDefault()
        window.contract.add_resource({url:url, resource_type:resource_type, budget:target * 1, description:description})
        setShowNotification(!showNotification)
        alert(`event info: ${url} ${resource_type}  ${target} ${description}`)
      }
    console.log(`its ${toggleModal}`);
      return (
        <div>
          {toggleModal == true && (
            <div className='addevent'>
              <form onSubmit={handleSubmit}>
                <label>
                  Enter resource url:
                  <input
                    type="text"
                    value={url}
                    onChange={(e) => setUrl(e.target.value)}
                  />
                </label><br></br>
                <label>
                  Enter resource type (tutorial, docs, course):
                  <input
                    type="text"
                    value={resource_type}
                    onChange={(e) => setType(e.target.value)}
                  />
                </label><br></br>
                <label>
              Enter budget:
              <input
                type="number"
                value={target}
                onChange={(e) => setTarget(e.target.value)}
              />
            </label><br></br>
                <label>
                  Enter event description:
                  <input
                    type="text"
                    value={description}
                    onChange={(e) => setDescription(e.target.value)}
                  />
                </label><br></br>
                <button  className='submit' >Submit</button>
              </form>
            </div>
          )}
          
          {showNotification && <Notification />}
        </div>
        
      )
    }
    function Notification() {
      return (
        <aside>
          <footer>
            <div>✔ Succeeded </div> 
            <div>New resource added. Thank you!</div>
          </footer>
        </aside>
      )
    }
    export default AddResource