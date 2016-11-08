package me.telegram.borken_bot.commands;

import lib.TestUtils;
import org.junit.Before;
import org.junit.Test;

import java.util.HashMap;
import java.util.Map;

import static org.junit.Assert.*;

public class DiceTest {
    private Dice dice;

    @Before
    public void setUp() {
        dice = new Dice();
    }

    @Test
    public void testGetMessageWithModifier() {
        Map<String, String> group = new HashMap<>();

        group.put("max", "1");
        group.put("modifier", "3");

        assertEquals("4", dice.getMessage(group));
    }

    @Test
    public void testGetMessageWithMaxOnly() {
        Map<String, String> group = new HashMap<>();
        group.put("max", "1");
        assertEquals("1", dice.getMessage(group));
    }

    @Test
    public void testGetMessageWithCount() {
        Map<String, String> group = new HashMap<>();
        group.put("count", "2");
        group.put("max", "1");
        assertEquals("2", dice.getMessage(group));
    }

    @Test
    public void testGetMessageWithoutMax() {
        Map<String, String> group = new HashMap<>();
        int actual = Integer.parseInt(dice.getMessage(group));
        assertTrue(actual >= 1 && actual <= 20);
    }

    @Test
    public void testTokenForShortWithDefault() {
        Map<String, String> groups = dice.tokenize("dice");

        assertEquals(null, groups.get("count"));
        assertEquals("20", groups.get("max"));
        assertEquals(null, groups.get("modifier"));
    }

    @Test
    public void testTokenForShortWithAll() {
        Map<String, String> groups = dice.tokenize("3 d  \n6 + 5");

        assertEquals("3", groups.get("count"));
        assertEquals("6", groups.get("max"));
        assertEquals("5", groups.get("modifier"));
    }

    @Test
    public void testTokenForShortWithModifier() {
        Map<String, String> groups = dice.tokenize("d8+2");

        assertEquals(null, groups.get("count"));
        assertEquals("8", groups.get("max"));
        assertEquals("2", groups.get("modifier"));
    }

    @Test
    public void testTokenForShortWithCount() {
        Map<String, String> groups = dice.tokenize("2d4");

        assertEquals("2", groups.get("count"));
        assertEquals("4", groups.get("max"));
        assertEquals(null, groups.get("modifier"));
    }

    @Test
    public void testTokenForShortWithMax() {
        Map<String, String> groups = dice.tokenize("d20");

        assertEquals(null, groups.get("count"));
        assertEquals("20", groups.get("max"));
        assertEquals(null, groups.get("modifier"));
    }

    @Test
    public void testTokenForLongWithModifier() {
        Map<String, String> groups = dice.tokenize("dice20+1");

        assertEquals(null, groups.get("count"));
        assertEquals("20", groups.get("max"));
        assertEquals("1", groups.get("modifier"));
    }

    @Test
    public void testTokenForLongWithMax() {
        Map<String, String> groups = dice.tokenize("dice20");

        assertEquals(null, groups.get("count"));
        assertEquals("20", groups.get("max"));
        assertEquals(null, groups.get("modifier"));
    }

    @Test
    public void testConflictCommands() {
        assertFalse(TestUtils.isConflict(dice));
    }
}