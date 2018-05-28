import diff from 'virtual-dom/diff'
import patch from 'virtual-dom/patch'
import createElement from 'virtual-dom/create-element'
import VPatch from 'virtual-dom/vnode/vpatch'

import * as bridge from 'vdom-rsjs'

function start() {
  let tree, rootNode
  let socket = new WebSocket("ws://localhost:8080", ["vdom-rsjs-websocket"])

  let onaction = (event, action) => {
    socket.send(JSON.stringify(action))
  }

  socket.onclose = () => {
    document.body.innerHTML += "<br>socket closed"
  }

  socket.onerror = err => {
    document.body.innerHTML += "<br>socket error: "
    document.body.innerHTML += "<br>" + err.toString()
    document.body.innerHTML += "<br>" + JSON.stringify(err)
  }

  socket.onopen = () => {
    document.body.innerHTML = "socket opened"
  }

  socket.onmessage = msg => {
    try {
      msg = JSON.parse(msg.data)
      if (msg.tree) {
        if (rootNode) {
          let newTree = bridge.node(msg.tree, onaction)
          let patches = diff(tree, newTree)
          tree = newTree
          rootNode = patch(rootNode, patches)
        } else {
          tree = bridge.node(msg.tree, onaction)
          rootNode = createElement(tree)
          document.body.appendChild(rootNode)
        }
      } else {
        document.body.innerHTML += "<br>message without property"
      }
    } catch (err) {
      document.body.innerHTML += "<br>onmessage error: "
      document.body.innerHTML += "<br>" + err.toString()
      document.body.innerHTML += "<br>" + JSON.stringify(err)
      document.body.innerHTML += "<br>msg: " + msg.data
    }
  }

  window.onmessage = msg => {
      
  }
}

start();
