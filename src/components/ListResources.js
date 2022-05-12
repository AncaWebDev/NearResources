import React from 'react'

function ListResources({ project }) {

  return (
    <div className="project">
    <h2><a href={project.url} target="_blank">{project.url}</a></h2>{' '}
      <h3>Description:</h3>
      <p>{project.description}</p>
      <h4>Type: {project.resource_type}</h4>
      <h4>Added by: {project.creator}</h4>
      <h4>Votes: {project.total_votes}</h4>
      <button
        onClick={() => {
          window.contract.vote({ id: project.id })
        }}
      >
        Vote
      </button>
    </div>
  )
}

export default ListResources