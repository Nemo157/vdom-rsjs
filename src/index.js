import { VNode, VText, VPatch } from 'virtual-dom'

function omap(o, f) {
  let n = {};
  for (const [k, v] of Object.entries(o)) {
    n[k] = f(v)
  }
  return n
}

export function props(data, onaction) {
  return omap(data, value => {
    if (value.hasOwnProperty('action')) {
      return (event) => onaction(event, value.action)
    } else if (value.hasOwnProperty('text')) {
      return value.text
    } else if (value.hasOwnProperty('object')) {
      return value.object
    }
  })
}

export function node(data, onaction) {
  if (data.hasOwnProperty('tag')) {
    return new VNode(
      data.tag.name,
      props(data.tag.properties, onaction),
      data.tag.children.map(child => node(child, onaction)),
      data.tag.key,
      data.tag.namespace)
  } else if (data.hasOwnProperty('text')) {
    return new VText(data.text)
  }
}

export function patch(data, onaction) {
  if (data.hasOwnProperty('none')) {
    return new VPatch(VPatch.NONE)
  } else if (data.hasOwnProperty('text')) {
    return new VPatch(VPatch.VTEXT, null, data.text)
  } else if (data.hasOwnProperty('node')) {
    return new VPatch(VPatch.VNODE, null, data.node)
  } else if (data.hasOwnProperty('props')) {
    let previous = props(data.props.prev, onaction);
    let next = props(data.props.next, onaction);
    return new VPatch(VPatch.PROPS, { previous }, next)
  } else if (data.hasOwnProperty('reorder')) {
    return new VPatch(VPatch.ORDER, null, data.reorder)
  } else if (data.hasOwnProperty('insert')) {
    return new VPatch(VPatch.INSERT, null, data.insert)
  } else if (data.hasOwnProperty('remove')) {
    return new VPatch(VPatch.REMOVE, data.remove, null)
  }
}
