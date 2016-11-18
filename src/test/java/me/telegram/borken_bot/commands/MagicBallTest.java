package me.telegram.borken_bot.commands;

import lib.TestUtils;
import org.junit.Before;
import org.junit.Test;

import static org.junit.Assert.*;

public class MagicBallTest {
    private MagicBall ball;

    @Before
    public void setUp() {
        ball = new MagicBall();
    }

    @Test
    public void testIsValidAction() {
        assertTrue(ball.isValidAction("8", null));
        assertTrue(ball.isValidAction("ball", null));
        assertTrue(ball.isValidAction("8ball", null));
        assertFalse(ball.isValidAction("_8ball", null));
    }

    @Test
    public void testGetShortNotations() {
        String[] notations = ball.getShortNotations();

        assertEquals(2, notations.length);
        assertEquals("8", notations[0]);
        assertEquals("ball", notations[1]);
    }

    @Test
    public void testConflictCommands() {
        assertFalse(TestUtils.isConflict(ball));
    }
}