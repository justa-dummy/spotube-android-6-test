/*
 * Copyright (C) 2026 Kingkor Roy Tirtho and Spotube Contributors
 *
 * This program is free software: you can redistribute it and/or modify
 * it under the terms of the GNU Affero General Public License as published by
 * the Free Software Foundation, either version 3 of the License, or
 * (at your option) any later version.
 *
 * This program is distributed in the hope that it will be useful,
 * but WITHOUT ANY WARRANTY; without even the implied warranty of
 * MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
 * GNU Affero General Public License for more details.
 *
 * You should have received a copy of the GNU Affero General Public License
 * along with this program.  If not, see <https://www.gnu.org/licenses/>.
 */
package org.spotube.spotube.modules.shell

import androidx.lifecycle.Lifecycle

/**
 * Lifecycle state below which the player UI stops collecting the playback position (it ticks every 250 ms
 * and recomposes the whole player). On Android the activity is only paused, not stopped, when the screen
 * turns off without a lock screen, so STARTED would keep recomposing in the dark.
 */
internal expect val playerPositionMinActiveState: Lifecycle.State
