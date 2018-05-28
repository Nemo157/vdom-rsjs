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
    if (value.action) {
      return (event) => onaction(event, value.action)
    } else if (value.text) {
      return value.text
    } else if (value.object) {
      return value.object
    }
  })
}

export function node(data, onaction) {
  if (data.tag) {
    return new VNode(
      data.tag.name,
      props(data.tag.properties, onaction),
      data.tag.children.map(child => node(child, onaction)),
      data.tag.key,
      data.tag.namespace)
  } else if (data.text) {
    return new VText(data.text)
  }
}

export function patch(data, onaction) {
  if (data.none) {
    return new VPatch(VPatch.NONE)
  } else if (data.text) {
    return new VPatch(VPatch.VTEXT, null, data.text)
  } else if (data.node) {
    return new VPatch(VPatch.VNODE, null, data.node)
  } else if (data.props) {
    let previous = props(data.props.prev, onaction);
    let next = props(data.props.next, onaction);
    return new VPatch(VPatch.PROPS, { previous }, next)
  } else if (data.reorder) {
    return new VPatch(VPatch.ORDER, null, data.reorder)
  } else if (data.insert) {
    return new VPatch(VPatch.INSERT, null, data.insert)
  } else if (data.remove) {
    return new VPatch(VPatch.REMOVE, data.remove, null)
  }
}
