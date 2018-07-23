class GameUtil {
  // calc AABB(Axis-Aligned Bounding Box) closest point (all point is world point)
  static calcNearPointOnAABB(targetPoint, minBoxPoint, maxBoxPoint) {
    let nearPoint = targetPoint.clone();

    if (nearPoint.x < minBoxPoint.x) {
      nearPoint.x = minBoxPoint.x;
    } else if (nearPoint.x > maxBoxPoint.x) {
      nearPoint.x = maxBoxPoint.x;
    }

    if (nearPoint.y < minBoxPoint.y) {
      nearPoint.y = minBoxPoint.y;
    } else if (nearPoint.y > maxBoxPoint.y) {
      nearPoint.y = maxBoxPoint.y;
    }

    if (nearPoint.z < minBoxPoint.z) {
      nearPoint.z = minBoxPoint.z;
    } else if (nearPoint.z > maxBoxPoint.z) {
      nearPoint.z = maxBoxPoint.z;
    }

    return nearPoint;
  }

  // convert to world point from local point
  static toWorldPoint(basePoint, localPoint) {
    let worldPoint = basePoint.clone();
    return worldPoint.add(localPoint);
  }
}
